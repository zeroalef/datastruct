use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

type Link<T> = Option<NonNull<OneLinkNode<T>>>;

struct OneLinkNode<T> {
    next: Link<T>,
    data: T,
}

/// One-linked list implementation also known as `stack`.
/// Fully imperative implementation using pointers for faster.

/// not thread safety list
pub struct LinkedList<T>(LinkedListInner<T>);

impl<T> LinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        Self(LinkedListInner::new())
    }
    #[inline]
    pub fn push(&mut self, data: T) {
        self.0.cons(data);
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.car()
    }
    #[inline]
    pub fn rest(&self) -> Self {
        Self(self.0.cdr())
    }
}

/// thread safety list
pub struct TSLinkedList<T>(Arc<Mutex<LinkedListInner<T>>>);

impl<T> TSLinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(LinkedListInner::new())))
    }
    #[inline]
    pub fn push(&mut self, data: T) {
        self.0.lock().unwrap().cons(data);
    }
    #[inline]
    pub fn pop(&self) -> T {
        self.0.lock().unwrap().car()
    }
    #[inline]
    pub fn rest(&self) -> Self {
        Self(Arc::new(Mutex::new(self.0.lock().unwrap().cdr())))
    }
}

struct LinkedListInner<T> {
    head: Link<T>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> LinkedListInner<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
            _marker: PhantomData,
        }
    }
    #[inline(always)]
    pub fn cons(&mut self, data: T) {
        unsafe {
            let new_node =
                NonNull::new_unchecked(Box::into_raw(Box::new(OneLinkNode { next: None, data })));
            (*new_node.as_ptr()).next = self.head;
            self.head = Some(new_node);
            self.len += 1;
        }
    }
    #[inline(always)]
    pub fn car(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_value = boxed.data;
                self.head = boxed.next;
                self.len -= 1;
                return_value
            })
        }
    }
    #[inline(always)]
    pub fn cdr(&self) -> Self {
        unsafe {
            let mut ret_list = Self::new();
            if let Some(node) = self.head {
                ret_list.head = (*node.as_ptr()).next;
                ret_list.len = self.len - 1;
            }
            ret_list
        }
    }
}

#[cfg(test)]
mod test {
    use crate::list::imperative_list::LinkedListInner;
    #[test]
    fn test_linked_list() {
        let mut l: LinkedListInner<i32> = LinkedListInner::new();
        (0..1000000).into_iter().for_each(|value| {
            l.cons(value);
        });
        (0..1000000).into_iter().rev().for_each(|value| {
            assert_eq!(l.car(), Some(value));
        });
    }
}
