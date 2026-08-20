use crate::hipfft;
use cuda_types::cufft::{cufftError_t, cufftHandle, cufftResult};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

enum BackendPlan {
    HipFft(hipfft::Handle),
}

// hipFFT plans are used only while their per-plan mutex is held.
unsafe impl Send for BackendPlan {}

type Plan = Arc<Mutex<Option<BackendPlan>>>;

struct Registry {
    next_handle: cufftHandle,
    plans: HashMap<cufftHandle, Plan>,
}

impl Registry {
    fn new() -> Self {
        Self {
            next_handle: 1,
            plans: HashMap::new(),
        }
    }

    fn insert(&mut self, backend: hipfft::Handle) -> Result<cufftHandle, cufftError_t> {
        let handle = self.next_handle;
        self.next_handle = self
            .next_handle
            .checked_add(1)
            .ok_or(cufftError_t::ALLOC_FAILED)?;
        self.plans.insert(
            handle,
            Arc::new(Mutex::new(Some(BackendPlan::HipFft(backend)))),
        );
        Ok(handle)
    }

    fn get(&self, handle: cufftHandle) -> Result<Plan, cufftError_t> {
        self.plans
            .get(&handle)
            .cloned()
            .ok_or(cufftError_t::INVALID_PLAN)
    }

    fn remove(&mut self, handle: cufftHandle) -> Result<Plan, cufftError_t> {
        self.plans.remove(&handle).ok_or(cufftError_t::INVALID_PLAN)
    }
}

fn registry() -> &'static Mutex<Registry> {
    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

pub(crate) fn insert(backend: hipfft::Handle) -> Result<cufftHandle, cufftError_t> {
    registry()
        .lock()
        .map_err(|_| cufftError_t::INTERNAL_ERROR)?
        .insert(backend)
}

pub(crate) fn with(
    handle: cufftHandle,
    operation: impl FnOnce(hipfft::Handle) -> cufftResult,
) -> cufftResult {
    let plan = registry()
        .lock()
        .map_err(|_| cufftError_t::INTERNAL_ERROR)?
        .get(handle)?;
    let plan = plan.lock().map_err(|_| cufftError_t::INTERNAL_ERROR)?;
    let backend = plan.as_ref().ok_or(cufftError_t::INVALID_PLAN)?;
    match backend {
        BackendPlan::HipFft(handle) => operation(*handle),
    }
}

pub(crate) fn remove(
    handle: cufftHandle,
    destroy: impl FnOnce(hipfft::Handle) -> cufftResult,
) -> cufftResult {
    let plan = registry()
        .lock()
        .map_err(|_| cufftError_t::INTERNAL_ERROR)?
        .remove(handle)?;
    let mut plan = plan.lock().map_err(|_| cufftError_t::INTERNAL_ERROR)?;
    let backend = plan.take().ok_or(cufftError_t::INVALID_PLAN)?;
    match backend {
        BackendPlan::HipFft(handle) => destroy(handle),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ptr, sync::mpsc, thread, time::Duration};

    #[test]
    fn public_handles_are_nonzero_and_independent_of_backend_pointer() {
        let mut registry = Registry::new();
        let backend = 0x1000usize as hipfft::Handle;
        let first = registry.insert(backend).unwrap();
        let second = registry.insert(backend).unwrap();
        assert_ne!(first, 0);
        assert_ne!(first, second);
        assert_ne!(first as usize, backend as usize);
    }

    #[test]
    fn invalid_and_removed_handles_are_rejected() {
        let mut registry = Registry::new();
        assert!(matches!(registry.get(0), Err(cufftError_t::INVALID_PLAN)));
        let handle = registry.insert(ptr::dangling_mut()).unwrap();
        registry.remove(handle).unwrap();
        assert!(matches!(
            registry.get(handle),
            Err(cufftError_t::INVALID_PLAN)
        ));
        assert!(matches!(
            registry.remove(handle),
            Err(cufftError_t::INVALID_PLAN)
        ));
    }

    #[test]
    fn destroy_waits_for_an_in_flight_operation() {
        let handle = insert(ptr::dangling_mut()).unwrap();
        let entered = Arc::new(std::sync::Barrier::new(2));
        let release = Arc::new(std::sync::Barrier::new(2));
        let operation = {
            let entered = Arc::clone(&entered);
            let release = Arc::clone(&release);
            thread::spawn(move || {
                with(handle, |_| {
                    entered.wait();
                    release.wait();
                    Ok(())
                })
            })
        };

        entered.wait();
        let (destroyed, receiver) = mpsc::channel();
        let destroy = thread::spawn(move || {
            let result = remove(handle, |_| Ok(()));
            destroyed.send(result).unwrap();
        });
        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());

        release.wait();
        assert_eq!(operation.join().unwrap(), Ok(()));
        assert_eq!(
            receiver.recv_timeout(Duration::from_secs(1)).unwrap(),
            Ok(())
        );
        destroy.join().unwrap();
        assert_eq!(with(handle, |_| Ok(())), Err(cufftError_t::INVALID_PLAN));
    }
}
