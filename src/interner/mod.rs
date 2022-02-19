// need to use until get_or_insert is made stable
use hashbrown::{HashMap, HashSet};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::intrinsics::transmute;
use std::slice::SliceIndex;
use std::sync::{Arc, Mutex};

pub type Interned<T> = &'static T;

pub fn intern<T>(value: T) -> Interned<T>
where
    T: Internable,
{
    T::intern(value)
}

pub fn leak<T>(value: T) -> &'static T {
    let boxed = Box::new(value);
    Box::leak(boxed)
}

pub struct Leaked<T: 'static> {
    leaked: &'static T,
}

impl<T: 'static> Leaked<T> {
    pub fn new(value: T) -> Self {
        Self {
            leaked: leak(value),
        }
    }
}

pub trait Interner<T> {
    fn intern(&self, value: T) -> &T;
}

pub struct HashMapInterner<'a, T>
where
    T: Internable,
    //T: 'static
{
    map: Mutex<HashMap<&'a T, &'a T>>,
}

pub struct HashSetInterner<T>
where
    T: Internable,
    //T: 'static
{
    set: Mutex<HashSet<T>>,
}

impl<T> Interner<T> for HashSetInterner<T>
where
    T: Internable,
{
    fn intern(&self, value: T) -> &T {
        match self.set.lock() {
            Ok(mut set) => unsafe {
                // the lock sets the lifetime to '_
                // but i know the set will be around as long as the outer bit
                // so i transmute it
                transmute(set.get_or_insert(value))
            },
            Err(_) => unimplemented!("can't handle posioned"),
        }
    }
}

impl<T> Default for HashSetInterner<T>
where
    T: Internable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HashSetInterner<T>
where
    T: Internable,
{
    pub fn new() -> Self {
        let set = HashSet::new();
        let set = Mutex::new(set);
        HashSetInterner { set }
    }
}

pub trait Internable: Sized + Hash + Eq {
    type Interner: Interner<Self> + Default;
    fn intern(value: Self) -> Interned<Self>;
}

macro_rules! make_internable {
    ($ty:path, $ii:ident) => {
        //interner
        lazy_static::lazy_static! {
            static ref $ii: <$ty as Internable>::Interner = Default::default();
        }

        impl Internable for $ty {
            // type Hasher = DefaultHasher;
            type Interner = HashSetInterner<$ty>;
            // type Key = Signal;

            fn intern(value: Self) -> Interned<Self> {
                $ii.intern(value)
            }
        }
    };
}

make_internable!(String, STRING_INTERNER);

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::interner::intern;
    use crate::signal::Signal;

    #[test]
    fn test_intern_string() {
        let interned = intern("hello world".to_string());
        let cool = interned as *const _;
        println!("cool {:?} {}", cool, interned);

        for i in 1..200_000 {
            intern(format!("intern{}", i));
        }
        println!("cool {:?} {}", cool, interned);

        let interned = intern("hello world".to_string());
        let cool = interned as *const _;
        println!("cool {:?} {}", cool, interned);

    }
}
