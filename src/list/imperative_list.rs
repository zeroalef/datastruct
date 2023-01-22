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
        self.0.head()
    }
    #[inline]
    pub fn touch(&mut self) -> Option<T> {
        self.0.car()
    }
    #[inline]
    pub fn rest(&self) -> Self {
        Self(self.0.cdr())
    }
}

/// thread safety list
pub struct TSLinkedList<T>(Arc<Mutex<LinkedListInner<T>>>);

impl<T> Clone for TSLinkedList<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

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
    pub fn pop(&self) -> Option<T> {
        self.0.lock().unwrap().head()
    }
    #[inline]
    pub fn touch(&self) -> Option<T> {
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

impl<T> Drop for LinkedListInner<T> {
    fn drop(&mut self) {
        while self.head().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedListInner<T> {}
unsafe impl<T: Sync> Sync for LinkedListInner<T> {}

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
    pub fn is_empty(&self) -> bool {self.len == 0}
    #[inline(always)]
    pub fn len(&self) -> usize {self.len}

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
    pub fn head(&mut self) -> Option<T> {
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
    pub fn car(&mut self) -> Option<T> {
        unsafe { self.head.map(|node| Box::from_raw(node.as_ptr()).data) }
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
mod test_linked_list {
    use crate::list::imperative_list::LinkedList;
    use crate::list::imperative_list::LinkedListInner;
    use crate::list::imperative_list::TSLinkedList;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_linked_list_inner() {
        let mut l: LinkedListInner<i32> = LinkedListInner::new();
        (0..1000).into_iter().for_each(|value| {
            l.cons(value);
        });
        (0..1000).into_iter().rev().for_each(|value| {
            assert_eq!(l.head(), Some(value));
        });
    }
    #[test]
    fn test_linked_list() {
        let mut l: LinkedList<i32> = LinkedList::new();
        (0..1000).into_iter().for_each(|value| {
            l.push(value);
        });
        (0..1000).into_iter().rev().for_each(|value| {
            assert_eq!(l.pop(), Some(value));
        });
    }
    #[test]
    fn test_thread_safe_linked_list() {
        let mut list: TSLinkedList<u64> = TSLinkedList::new();
        let mut list_ref = list.clone();

        thread::scope(move |s| {
            let t1 = Duration::from_millis(11);
            let t2 = Duration::from_millis(23);
            s.spawn(move || {
                (0..100).into_iter().for_each(|x| {
                    list.push(x);
                    thread::sleep(t1);
                });
            });
            s.spawn(move || {
                (0..100).into_iter().for_each(move |x| {
                    thread::sleep(t2);
                    match list_ref.pop() {
                        Some(val) => println!("{}", val),
                        None => println!("None"),
                    }
                });
            });
        });
    }
}
