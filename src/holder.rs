use std::{
    cell::{Cell, RefCell, UnsafeCell},
    intrinsics::transmute,
    mem::MaybeUninit,
    ops::DerefMut,
    pin::Pin,
    ptr::NonNull,
    sync::{
        atomic::{AtomicPtr, AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
};

// a concurent vec that can only be inserted
// but gives refs that last a lifetime
trait Holder<T> {
    fn size(&self) -> usize;
    fn hold(&self, value: T) -> &T;
}

const BLOCK_SIZE: usize = 32;

// maybe use https://docs.rs/crossbeam/0.8.1/crossbeam/queue/struct.SegQueue.html

pub struct BlockHolder<T> {
    size: AtomicUsize,
    num_blocks: AtomicUsize,
    current: AtomicPtr<Block<T>>,

    // need AtomicCell<Block<T>> that can do .apply()
    blocks: RwLock<Vec<UninitArray<T, BLOCK_SIZE>>>,
}

// allocate new block
// if last is not none, set block to new
// swap last with

use crossbeam::atomic::AtomicCell;
use lazy_static::__Deref;

struct Block<T> {
    depth: usize,
    data: UninitArray<T, BLOCK_SIZE>,
    previous: Option<Arc<Block<T>>>,
}

impl<T> Block<T> {
    fn new(previous: Box<Block<T>>) -> Self {
        Self {
            depth: previous.depth + 1,
            data: Default::default(),
            previous: Some(previous),
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct UninitArray<T, const SIZE: usize> {
    //vec: Vec<UnsafeCell<T>>,
    data: Pin<Box<MaybeUninit<[Cell<T>; SIZE]>>>,
}

impl<T, const SIZE: usize> Default for UninitArray<T, SIZE> {
    fn default() -> Self {
        Self {
            data: Box::pin(MaybeUninit::uninit()),
        }
    }
}

impl<T, const SIZE: usize> UninitArray<T, SIZE> {
    pub fn set<'a>(&'a self, index: usize, value: T) -> &'a T {
        let array = unsafe { self.data.assume_init_ref() };
        let cell = &array[index];
        cell.set(value);
        unsafe { cell.as_ptr().as_ref().unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holder() {
        let block: UninitArray<String, 32> = UninitArray::default();
        let nice = block.set(0, "hello".to_string());
        println!("{}", nice)
    }
}

impl<T> BlockHolder<T> {
    fn capacity(&self) -> usize {
        self.blocks.read().unwrap().len() * BLOCK_SIZE
    }
    fn current(&self) -> &Block<T> {
        let nice = self.current.load(Ordering::Relaxed);
        let uwu = unsafe { nice.as_ref() }.unwrap();
        uwu
    }

    fn expand(&self) {
        // can do Box::new_zeroed() or unint
        //self.blocks.write().unwrap().push(UninitArray::default());
        let last_raw = self.current.load(Ordering::Relaxed);
        let last = unsafe { Box::from_raw(last_raw) };

        let next = Box::into_raw(Box::new(Block::new(last)));
        let hapend =
            self.current
                .compare_exchange(last_raw, next, Ordering::SeqCst, Ordering::Relaxed);
        if let Err(wrong_boi) = hapend {
            // i don't care about this
            // but i want to
        }
    }

    pub fn insert(&self, value: T) -> &T {
        let index = self.size.fetch_add(1, Ordering::Relaxed);
        if index >= self.capacity() {
            self.expand()
        }
        let cell_to_use = index / BLOCK_SIZE;
        let slot = index % BLOCK_SIZE;
        let block = &self.blocks.read().unwrap()[cell_to_use];
        let out = block.set(slot, value);
        unsafe { transmute(out) }
    }
}



// would be better as AtomicArc
#[repr(transparent)]
pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicBox<T> {
    pub fn new(from: T) -> Self {
        Self {
            ptr: AtomicPtr::new(Box::into_raw(Box::new(from))),
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
        //let start_ref = self.deref_mut();
        let start_raw = self.raw();
        // this is unsafe. what if apply at same time, but first one next takes longer to
        //let start_ref = unsafe { start_raw.as_ref().unwrap() };
        let next = next(start_raw);
        let next_box = Box::new(next);
        self.compare_exchange(start_raw, next_box)
    }
}
impl<T> std::ops::DerefMut for AtomicBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let raw = self.ptr.load(Ordering::Relaxed);
        unsafe { raw.as_mut() }.unwrap()
    }
}

impl<T> std::ops::Deref for AtomicBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let raw = self.ptr.load(Ordering::Relaxed);
        unsafe { raw.as_ref() }.unwrap()
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        let raw = self.raw();
        let boxed = unsafe { Box::from_raw(raw) };
        drop(boxed)
    }
}
