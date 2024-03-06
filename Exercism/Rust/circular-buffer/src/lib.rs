use std::{collections::btree_map::IterMut, marker::PhantomData, ptr::null};

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    phantom: PhantomData<T>,
    array: Vec<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            phantom: PhantomData,
            array: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.array.len() == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.array.push(_element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let result = self.array[0];
        self.array.remove(0);
        return Ok(result);
    }

    pub fn clear(&mut self) {
        self.array.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        todo!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
