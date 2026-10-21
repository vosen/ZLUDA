//! CUDA contexts: the registry of live contexts (the primary one and those `cuCtxCreate` made) and each thread's
//! stack of current contexts. Generic over the per-context state, so that the bookkeeping is tested without a GPU.

use std::cell::RefCell;
use std::collections::HashMap;

/// The live contexts, by handle.
pub(crate) struct Contexts<T> {
    states: HashMap<usize, T>,
    /// The primary context's handle (the same across `reset`), and how many retains it has.
    primary: usize,
    primary_refs: u32,
}

impl<T> Contexts<T> {
    pub(crate) fn new(primary: usize, state: T) -> Self {
        Contexts { states: HashMap::from([(primary, state)]), primary, primary_refs: 0 }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn primary(&self) -> usize {
        self.primary
    }

    pub(crate) fn get_mut(&mut self, h: usize) -> Option<&mut T> {
        self.states.get_mut(&h)
    }

    pub(crate) fn contains(&self, h: usize) -> bool {
        self.states.contains_key(&h)
    }

    pub(crate) fn insert(&mut self, h: usize, state: T) {
        self.states.insert(h, state);
    }

    /// Removes a context made by `cuCtxCreate`; the primary one is only ever reset.
    pub(crate) fn remove(&mut self, h: usize) -> Option<T> {
        if h == self.primary {
            return None;
        }
        self.states.remove(&h)
    }

    pub(crate) fn retain_primary(&mut self) -> usize {
        self.primary_refs += 1;
        self.primary
    }

    pub(crate) fn release_primary(&mut self) -> Result<(), ()> {
        self.primary_refs = self.primary_refs.checked_sub(1).ok_or(())?;
        Ok(())
    }

    /// Replaces the primary context's state (cuDevicePrimaryCtxReset); the old one is returned to be dropped.
    pub(crate) fn reset_primary(&mut self, state: T) -> Option<T> {
        self.states.insert(self.primary, state)
    }

    /// The context calls on this thread use: the top of its stack, else the primary context (the CUDA runtime makes
    /// the primary context current before its first call; driver API programs that never set a context get it too).
    pub(crate) fn current(&self) -> usize {
        STACK.with(|s| s.borrow().last().copied()).filter(|h| self.contains(*h)).unwrap_or(self.primary)
    }
}

thread_local! {
    /// This thread's current contexts, the current one last.
    static STACK: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

/// The thread's current context as `cuCtxGetCurrent` reports it: none until one is made current.
pub(crate) fn top() -> Option<usize> {
    STACK.with(|s| s.borrow().last().copied())
}

pub(crate) fn push(h: usize) {
    STACK.with(|s| s.borrow_mut().push(h));
}

pub(crate) fn pop() -> Option<usize> {
    STACK.with(|s| s.borrow_mut().pop())
}

/// cuCtxSetCurrent: replaces the top of the stack (pushes onto an empty one); `None` pops it.
pub(crate) fn set(h: Option<usize>) {
    STACK.with(|s| {
        let mut s = s.borrow_mut();
        match h {
            Some(h) => match s.last_mut() {
                Some(top) => *top = h,
                None => s.push(h),
            },
            None => {
                s.pop();
            }
        }
    })
}

/// Drops a destroyed context from this thread's stack.
pub(crate) fn forget(h: usize) {
    STACK.with(|s| s.borrow_mut().retain(|&x| x != h));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test runs on its own thread, so the stacks do not interfere.
    #[test]
    fn the_stack_follows_push_pop_and_set() {
        let mut c = Contexts::new(1, "primary");
        assert_eq!(top(), None);
        assert_eq!(c.current(), 1, "no current context: the primary one");
        c.insert(7, "seven");
        push(7);
        assert_eq!((top(), c.current()), (Some(7), 7));
        set(Some(1));
        assert_eq!((top(), c.current()), (Some(1), 1), "set replaces the top");
        push(7);
        assert_eq!(pop(), Some(7));
        assert_eq!(pop(), Some(1));
        assert_eq!(pop(), None);
        set(Some(7));
        assert_eq!(top(), Some(7), "set on an empty stack pushes");
        set(None);
        assert_eq!(top(), None, "set(null) pops");
    }

    #[test]
    fn destroyed_contexts_leave_the_stack() {
        let mut c = Contexts::new(1, 10);
        c.insert(5, 50);
        push(5);
        assert_eq!(c.remove(5), Some(50));
        assert_eq!(c.current(), 1, "a context destroyed elsewhere falls back to the primary one");
        forget(5);
        assert_eq!(top(), None);
        assert_eq!(c.remove(1), None, "the primary context is reset, not removed");
        assert!(c.contains(1));
    }

    #[test]
    fn the_primary_context_is_retained_released_and_reset() {
        let mut c = Contexts::new(3, String::from("old"));
        assert_eq!(c.retain_primary(), 3);
        assert_eq!(c.retain_primary(), 3);
        assert_eq!(c.release_primary(), Ok(()));
        assert_eq!(c.release_primary(), Ok(()));
        assert_eq!(c.release_primary(), Err(()), "released more than retained");
        assert_eq!(c.reset_primary(String::from("new")).as_deref(), Some("old"));
        assert_eq!(c.get_mut(3).map(|s| s.as_str()), Some("new"));
        assert_eq!(c.primary(), 3, "the handle survives a reset");
    }
}
