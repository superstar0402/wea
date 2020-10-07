//! High-level library to store data in NVM memory
//!
//! This module provides primitives to store objects in the Flash memory used
//! by the application. It implements basic update mechanisms, eventually with
//! atomicity guarantees against possible tearing.
//!
//! There is no filesystem or NVM allocated in BOLOS. Therefore any object
//! stored by the application uses a fixed space in the program itself.
//!
//! # Examples
//!
//! The following piece of code declares a storage for an integer, with atomic
//! update:
//!
//! ```
//! // This is necessary to store the object in NVM and not in RAM
//! #[link_section=".nvm"]
//! static mut COUNTER: nvm::AtomicStorage<i32> = nvm::AtomicStorage::new(&3);
//! ```
//!
//! In the program, `COUNTER` must not be used directly. It is a static variable
//! and using it would require unsafe everytime. Instead, a reference must be
//! taken, so the borrow checker will be able to do its job correctly. This is
//! crucial: the memory location of the stored object may be moved due to
//! atomicity implementation, and the borrow checker should prevent any use of
//! old references to a value which has been updated and moved elsewhere.
//!
//! ```
//! let mut counter = unsafe { &mut COUNTER };
//! println!("counter value is {}", *counter.get_ref());
//! counter.update(&(*counter.get_ref() - 1));
//! println!("counter value is {}", *counter.get_ref());
//! ```

use crate::bindings::*;

// Warning: currently alignment is fixed by magic values everywhere, since
// rust does not allow using a constant in repr(align(...))
// This code will work correctly only for the currently set page size of 64.

/// What storage of single element should implement.
///
/// The address of the stored object, returned with get_ref, MUST remain the
/// same until update is called.
///
/// The update method may move the object, for instance with AtomicStorage.
///
/// The borrow checker should prevent keeping references after updating the
/// content, so everything should go fine...
pub trait SingleStorage<T> {
    /// Returns a non-mutable reference to the stored object.
    fn get_ref(&self) -> &T;
    fn update(&mut self, value: &T);
}

/// Wraps a variable stored in Non-Volatile Memory to provide read and update
/// methods.
///
/// Always aligned to the beginning of a page to prevent different
/// AlignedStorage sharing a common Flash page (this is required to implement
/// unfinished write detection in SafeStorage and atomic operations in
/// AtomicStorage).
///
/// Warning: this wrapper does not provide any garantee about update atomicity.
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct AlignedStorage<T> {
    /// Stored value.
    /// This is intentionally private to prevent direct write access (this is
    /// stored in Flash, so only the update method can change the value).
    value: T
}

impl<T> AlignedStorage<T> {
    /// Create a Storage<T> initialized with a given value.
    /// This is to set the initial value of static Storage<T>, as the value
    /// member is private.
    pub const fn new(value: T) -> AlignedStorage<T> {
        AlignedStorage { value: value }
    }
}

impl<T> SingleStorage<T> for AlignedStorage<T> {
    /// Return non-mutable reference to the stored value.
    /// The address is always the same for AlignedStorage.
    fn get_ref(&self) -> &T {
        &self.value
    }

    /// Update the value by writting to the NVM memory.
    /// Warning: this can be vulnerable to tearing - leading to partial write.
    fn update(&mut self, value: &T){
        unsafe {
            nvm_write(
                &self.value as *const T as *const cty::c_void
                    as *mut cty::c_void,
                value as *const T as *const cty::c_void as *mut cty::c_void,
                core::mem::size_of::<T>() as u32);
            let mut _dummy = &self.value;
        }
    }
}

/// Just a non-zero magic to mark a storage as valid, when the update procedure
/// has not been interupted. Any value excepted 0 and 0xff may work.
const STORAGE_VALID: u8 = 0xa5;

/// Non-Volatile data storage, with a flag to detect corruption if the update
/// has been interrupted somehow.
///
/// During update:
/// 1. The flag is reset to 0
/// 2. The value is updated
/// 3. The flag is restored to STORAGE_VALID
pub struct SafeStorage<T> {
    flag: AlignedStorage<u8>,
    value: AlignedStorage<T>
}

impl<T> SafeStorage<T> {
    pub const fn new(value: T) -> SafeStorage<T> {
        SafeStorage {
            flag: AlignedStorage::new(STORAGE_VALID),
            value: AlignedStorage::new(value)
        }
    }

    /// Set the validation flag to zero to mark the content as invalid.
    /// This used for instance by the atomic storage management.
    pub fn invalidate(&mut self) {
        self.flag.update(&0);
    }

    /// Returns true if the stored value is not corrupted, false if a previous
    /// update operation has been interrupted.
    pub fn is_valid(&self) -> bool {
        *self.flag.get_ref() == STORAGE_VALID
    }
}

impl<T> SingleStorage<T> for SafeStorage<T> {
    /// Return non-mutable reference to the stored value.
    /// Panic if the storage is not valid (corrupted).
    fn get_ref(&self) -> &T {
        assert_eq!(*self.flag.get_ref(), STORAGE_VALID);
        self.value.get_ref()
    }

    fn update(&mut self, value: &T) {
        self.flag.update(&0);
        self.value.update(value);
        self.flag.update(&STORAGE_VALID);
    }
}

/// Non-Volatile data storage with atomic update support.
/// Takes at minimum twice the size of the data to be stored, plus 2 bytes.
#[repr(align(64))]
pub struct AtomicStorage<T> {
    // We must keep the storage B in another page, so when we update the
    // storage A, erasing the page of A won't modify the storage for B.
    // This is currently garanteed by the alignment of AlignedStorage.
    storage_a: SafeStorage<T>,
    storage_b: SafeStorage<T>
    // We also accept situations where both storages are marked as valid, which
    // can happen with tearing. This is not a problem, and we consider the first
    // one is the "correct" one.
}
    
impl<T> AtomicStorage<T> where T: Copy {
    /// Create an AtomicStorage<T> initialized with a given value.
    pub const fn new(value: &T) -> AtomicStorage<T> {
        AtomicStorage {
            storage_a: SafeStorage::new(*value),
            storage_b: SafeStorage::new(*value),
        }
    }

    /// Tell which of both storages contains the latest valid data. Returns
    /// 0 for storage A, 1 for storage B. Panic if none of the storage are
    /// valid (data corruption), although data corruption shall not be
    /// possible with tearing.
    fn which(&self) -> u32 {
        if self.storage_a.is_valid() {
            0
        } else if self.storage_b.is_valid() {
            1
        } else {
            panic!("invalidated atomic storage");
        }
    }
}

impl<T> SingleStorage<T> for AtomicStorage<T> where T: Copy {
    /// Return reference to the stored value.
    fn get_ref(&self) -> &T {
        if self.which() == 0 {
            self.storage_a.get_ref()
        } else {
            self.storage_b.get_ref()
        }
    }
    
    /// Update the value by writting to the NVM memory.
    /// Warning: this can be vulnerable to tearing - leading to partial write.
    fn update(&mut self, value: &T){
        if self.which() == 0 {
            self.storage_b.update(value);
            self.storage_a.invalidate();
        } else {
            self.storage_a.update(value);
            self.storage_b.invalidate();
        }
    }
}

/// A Non-Volatile fixed-size collection of fixed-size items.
/// Items insertion and deletion are atomic.
/// Items update is not implemented because the atomicity of this operation
/// cannot be garanteed here.
pub struct Collection<T, const N: usize> {
    flags: AtomicStorage<[u8;N]>,
    slots: [AlignedStorage<T>;N]
}

impl<T, const N: usize> Collection<T, N> where T: Copy {
    pub const fn new(value: T) -> Collection<T, N> {
        Collection { 
            flags: AtomicStorage::new(&[0;N]),
            slots: [AlignedStorage::new(value);N]
        }
    }

    /// Finds and returns a reference to a free slot, or returns an error if
    /// all slots are allocated.
    fn find_free_slot(&self) -> Result<usize, ()> {
        for (i, e) in self.flags.get_ref().iter().enumerate() {
            if *e != STORAGE_VALID {
                return Ok(i);
            }
        }
        Err(())
    }

    /// Adds an item in the collection. Returns an error if there is not free
    /// slots.
    /// This operation is atomic.
    pub fn add(&mut self, value: &T) -> Result<(), ()> {
        match self.find_free_slot() {
            Ok(i) => {
                self.slots[i].update(value);
                let mut new_flags = *self.flags.get_ref();
                new_flags[i] = STORAGE_VALID;
                self.flags.update(&new_flags);
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    /// Returns true if the indicated slot is allocated, or false if it is
    /// free.
    pub fn is_allocated(&self, index: usize) -> bool {
        self.flags.get_ref()[index] == STORAGE_VALID
    }

    /// Returns the number of allocated slots.
    pub fn len(&self) -> usize {
        let mut result = 0;
        for v in self.flags.get_ref() {
            if *v == STORAGE_VALID {
                result += 1;
            }
        }
        result
    }

    /// Returns the maximum number of items the collection can store.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the remaining number of items which can be added to the
    /// collection.
    pub fn remaining(&self) -> usize {
        self.capacity() - self.len()
    }

    /// Returns the index of an item in the internal storage, given the index
    /// in the collection. If index is too big, None is returned.
    ///
    /// # Arguments
    ///
    /// * `index` - Index in the collection
    fn index_to_key(&self, index: usize) -> Result<usize, ()> {
        let mut next = 0;
        let mut count = 0;
        loop {
            if next == N {
                return Err(())
            }
            if self.is_allocated(next) {
                if count == index {
                    return Ok(next);
                }
                count += 1
            }
            next += 1
        }
    }

    /// Returns reference to an item
    ///
    /// # Arguments
    ///
    /// * `index` - Item index
    pub fn get_ref(&self, index: usize) -> Result<&T, ()> {
        match self.index_to_key(index) {
            Ok(key) => Ok(self.slots[key].get_ref()),
            Err(()) => Err(())
        }
    }

    /// Removes an item from the collection.
    ///
    /// # Arguments
    ///
    /// * `index` - Item index
    pub fn remove(&mut self, index: usize) {
        let key = self.index_to_key(index).unwrap();
        let mut new_flags = *self.flags.get_ref();
        new_flags[key] = 0;
        self.flags.update(&new_flags);
    }

    /// Removes all the items from the collection.
    /// This operation is atomic.
    pub fn clear(&mut self) {
        self.flags.update(&[0;N]);
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Collection<T, N>
    where T: Copy
{
    type Item = &'a T;
    type IntoIter = CollectionIterator<'a, T, N>;

    fn into_iter(self) -> CollectionIterator<'a, T, N> {
        CollectionIterator { container: &self, next: 0 }
    }
}

pub struct CollectionIterator<'a, T, const N: usize> where T: Copy {
    container: &'a Collection<T, N>,
    next: usize
}

impl<'a, T, const N: usize> Iterator for CollectionIterator<'a, T, N>
    where T: Copy
{
    type Item = &'a T;

    fn next(&mut self) -> core::option::Option<&'a T> {
        loop {
            if self.next == N {
                return None
            }
            if self.container.is_allocated(self.next) {
                let result = Some(self.container.slots[self.next].get_ref());
                self.next += 1;
                return result;
            }
            self.next += 1;
        }
    }
}
