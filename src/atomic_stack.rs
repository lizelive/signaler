struct Node<T> {
    next: AtomicPtr<Node<T>>,
    element: T,
}

struct AtomicStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> AtomicStack<T> {
    fn push_front(&self, value: T);
    fn pop_front(&self) -> Option<T>;
}