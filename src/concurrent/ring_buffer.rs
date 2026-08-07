use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A power-of-two-capacity single-producer single-consumer ring buffer.
///
/// One slot is always left empty so full and empty are distinguishable.
pub struct SpscRingBuffer<T> {
    buf: Box<[UnsafeCell<MaybeUninit<T>>]>,
    mask: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Send> Send for SpscRingBuffer<T> {}
unsafe impl<T: Send> Sync for SpscRingBuffer<T> {}

impl<T> SpscRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two().max(2);
        let mut vec = Vec::with_capacity(cap);
        for _ in 0..cap {
            vec.push(UnsafeCell::new(MaybeUninit::uninit()));
        }

        Self {
            buf: vec.into_boxed_slice(),
            mask: cap - 1,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn capacity(&self) -> usize {
        self.mask + 1
    }

    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        tail.wrapping_sub(head) & self.mask
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity() - 1
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & self.mask;
        let head = self.head.load(Ordering::Acquire);

        if next_tail == head {
            return Err(item);
        }

        unsafe {
            let slot = self.buf[tail].get();
            (*slot).write(item);
        }

        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head == tail {
            return None;
        }

        let value = unsafe { (*self.buf[head].get()).assume_init_read() };
        self.head.store((head + 1) & self.mask, Ordering::Release);
        Some(value)
    }
}

impl<T> Drop for SpscRingBuffer<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
