use std::{sync::atomic::AtomicPtr, ptr::NonNull};

struct AtomicNotNull<T> {
    pointer: AtomicPtr<T>,
    nn: NonNull<T>
}
