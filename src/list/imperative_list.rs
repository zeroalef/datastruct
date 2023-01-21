use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<OneLinkNode<T>>>;

struct OneLinkNode<T> {
    next: Link<T>,
    data: T,
}

pub struct LinkedListInner<T> {
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
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(OneLinkNode {
                next: None,
                data: data,
            })));
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
        (0..100).into_iter().for_each(|value| {
            l.cons(value);
        });
        (0..100).into_iter().rev().for_each(|value| {
            assert_eq!(l.car(), Some(value));
        });
    }
}
