use std::sync::atomic::{AtomicBool, Ordering};

pub struct InitState {
    initialized: AtomicBool,
}

impl InitState {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(true),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    pub fn deinitialize(&self) {
        self.initialized.store(false, Ordering::SeqCst);
    }
}

#[macro_export]
macro_rules! define_init_state {
    ($name:ident) => {
        static $name: InitState = InitState::new();

        #[dtor::dtor]
        fn deinitialize() {
            INITIALIZED.deinitialize();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let state = InitState::new();
        assert_eq!(state.is_initialized(), true);
    }

    #[test]
    fn deinitialize() {
        let state = InitState::new();
        state.deinitialize();
        assert_eq!(state.is_initialized(), false);
    }
}
