use std::sync::atomic::{AtomicPtr, Ordering};

struct AtomicBox<T>{
    ptr: AtomicPtr<T>
}

impl<T> AtomicBox<T> {
    fn new(value: T){
        Self {
            ptr: AtomicPtr::new(Box::into_raw(Box::new(value))),
        }
    }

    pub fn raw(&self) -> *mut T {
        self.ptr.load(Ordering::Relaxed)
    }

    pub fn compare_exchange(&self, start_raw: *mut T, next_box: Box<T>) -> Result<Box<T>, Box<T>> {
        //let start_raw = self.raw();
        let next_raw = Box::into_raw(next_box);
        let exchange =
            self.ptr
                .compare_exchange(start_raw, next_raw, Ordering::SeqCst, Ordering::Relaxed);
        match exchange {
            Ok(_) => unsafe { Ok(Box::from_raw(start_raw)) },
            Err(_) => unsafe { Err(Box::from_raw(next_raw)) },
        }
    }

    pub fn try_apply<F>(&self, next: F) -> Result<Box<T>, Box<T>>
    where
        F: FnOnce(*mut T) -> T,
    {
        let start_raw = self.raw();
        let next = next(start_raw);
        let next_box = Box::new(next);
        self.compare_exchange(start_raw, next_box)
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        let raw = self.raw();
        let boxed = unsafe { Box::from_raw(raw) };
        drop(boxed)
    }
}