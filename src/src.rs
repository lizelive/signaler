use std::{sync::Arc, ptr::NonNull};

//https://docs.rs/arc-swap/latest/arc_swap/
#[repr(transparent)]
pub struct Asrc<T> {
    internal: AtomicBox<Arc<T>>,
}

impl<T> Asrc<T> {
    pub fn try_replace<F>(&self, next: F) -> Result<Arc<T>, Arc<T>>
    where
        F: FnOnce(Arc<T>) -> T,
    {
        let out = self
            .internal
            .try_apply(|last| {
                let last_arc = unsafe {
                    NonNull::new_unchecked(last)
                    .as_ref() // this doesnt increment the count
                    .clone() // this does
                };
                let strong = Arc::strong_count(&last_arc);
                assert_eq!(strong, 1, "pretty sure this is false, but true for testing");
                Arc::new(next(last_arc))
            })
            .map(|x| x.deref().clone())
            .map_err(|x| x.deref().clone());
        out
    }
}