use std::sync::Arc;

trait Chain {
    //fn with_prev(self, prev: Option<&Self>) -> Self;
    fn chain(&self, next: Self) -> &Self;
}

struct Swapper<T> {
    value: Box<T>,
}

impl<T> Swapper<T> {
    fn chain<F, G>(&self, make_next: F, chain: G)
    where
        F: FnOnce(&T) -> T,
        G: FnOnce(&mut T) -> &mut &T,
    {
        todo!()
    }
}


type NodeRef<T> = Arc<Node<T>>;
struct Node<T> {
    next: AtomicPtr<Node<T>>,
    prev: AtomicPtr<Node<T>>,
    element: T,
}

use std::collections::LinkedList;

use std::ptr::{NonNull, null_mut};
use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};

use crossbeam::channel::Select;

struct AtomicLinkedList<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
    len: AtomicUsize,
}

impl<T> AtomicLinkedList<T> {
    fn append(&self, value: T){
        let next = Box::new(
            Node{
                next: AtomicPtr::new(null_mut()),
                prev: AtomicPtr::new(null_mut()),
                element: value               
            }
        );

        // repeat until happy
        let head_raw = self.head.load(Ordering::Relaxed);
        next.prev.store(head_raw, Ordering::Relaxed);
        let next_raw = Box::into_raw(next);
        let head = unsafe {
            head_raw.as_ref().unwrap()
        }; 
        head.next.compare_exchange(null_mut(), next_raw, Ordering::Relaxed, Ordering::Relaxed);
    }
}



fn is_null<T>(ptr: &AtomicPtr<T>) {
    ptr.load(Ordering::Relaxed).is_null()
}