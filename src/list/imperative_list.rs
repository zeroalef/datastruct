pub mod stack {
    use std::marker::PhantomData;
    use std::ptr::NonNull;
    // use std::sync::{Arc, Mutex};

    type Link<T> = Option<NonNull<StackNode<T>>>;

    struct StackNode<T> {
        next: Link<T>,
        data: T,
    }

    /// One-linked list implementation also known as `stack`.
    /// Fully imperative implementation using pointers for faster.

    /// not thread safety list
    pub struct UnsafeStack<T>(LinkedListInner<T>);

    impl<T> UnsafeStack<T> {
        #[inline]
        pub fn new() -> Self {
            Self(LinkedListInner::new())
        }
        #[inline]
        pub fn push(&mut self, data: T) {
            self.0.push(data);
        }
        #[inline]
        pub fn pop(&mut self) -> Option<T> {
            self.0.pop()
        }
        #[inline]
        pub fn touch(&self) -> Option<&T> {
            self.0.touch()
        }
        #[inline]
        pub fn touch_mut(&mut self) -> Option<&mut T> {
            self.0.touch_mut()
        }
        // #[inline]
        // pub fn rest(&self) -> Self {
        //     Self(self.0.tail())
        // }
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        #[inline]
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    /// thread safety list
    // pub struct SafeStack<T>(Arc<Mutex<LinkedListInner<T>>>);
    //
    // impl<T> Clone for SafeStack<T> {
    //     fn clone(&self) -> Self {
    //         Self(Arc::clone(&self.0))
    //     }
    // }
    //
    // impl<T> SafeStack<T> {
    //     #[inline]
    //     pub fn new() -> Self {
    //         Self(Arc::new(Mutex::new(LinkedListInner::new())))
    //     }
    //     #[inline]
    //     pub fn push(&mut self, data: T) {
    //         self.0.lock().unwrap().push(data);
    //     }
    //     #[inline]
    //     pub fn pop(&self) -> Option<T> {
    //         self.0.lock().unwrap().pop()
    //     }
    //     #[inline]
    //     pub fn touch(&self) -> Option<T> {
    //         self.0.lock().unwrap().touch()
    //     }
    //     #[inline]
    //     pub fn rest(&self) -> Self {
    //         Self(Arc::new(Mutex::new(self.0.lock().unwrap().tail())))
    //     }
    //     #[inline]
    //     pub fn is_empty(&self) -> bool {
    //         self.0.lock().unwrap().is_empty()
    //     }
    //     #[inline]
    //     pub fn len(&self) -> usize {
    //         self.0.lock().unwrap().len()
    //     }
    // }

    /// One-linked list
    struct LinkedListInner<T> {
        head: Link<T>,
        len: usize,
        _marker: PhantomData<T>,
    }

    impl<T> Drop for LinkedListInner<T> {
        fn drop(&mut self) {
            while self.pop().is_some() {}
        }
    }

    // unsafe impl<T: Send> Send for LinkedListInner<T> {}
    // unsafe impl<T: Sync> Sync for LinkedListInner<T> {}

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
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
        #[inline(always)]
        pub fn len(&self) -> usize {
            self.len
        }

        #[inline(always)]
        pub fn push(&mut self, data: T) {
            unsafe {
                let new_node =
                    NonNull::new_unchecked(Box::into_raw(Box::new(StackNode { next: None, data })));
                (*new_node.as_ptr()).next = self.head;
                self.head = Some(new_node);
                self.len += 1;
            }
        }
        #[inline(always)]
        pub fn pop(&mut self) -> Option<T> {
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
        pub fn touch(&self) -> Option<&T> {
            unsafe { self.head.map(|node| &(*node.as_ptr()).data) }
        }
        #[inline(always)]
        pub fn touch_mut(&mut self) -> Option<&mut T> {
            // unsafe { self.head.map(|node| &(*node.as_ptr()).data) }
            unsafe { self.head.map(|node| &mut (*node.as_ptr()).data) }
        }
        // #[inline(always)]
        // pub fn tail(&self) -> Self {
        //     unsafe {
        //         let mut ret_list = Self::new();
        //         if let Some(node) = self.head {
        //             ret_list.head = (*node.as_ptr()).next;
        //             ret_list.len = self.len - 1;
        //         }
        //         ret_list
        //     }
        // }
    }

    #[cfg(test)]
    mod test_linked_list_inner {
        use super::LinkedListInner;
        #[test]
        fn test_linked_list_inner_push_pop() {
            let mut l: LinkedListInner<i32> = LinkedListInner::new();
            (0..1000).into_iter().for_each(|value| {
                l.push(value);
            });
            (0..1000).into_iter().rev().for_each(|value| {
                assert_eq!(l.pop(), Some(value));
            });
        }
        #[test]
        fn test_linked_list_inner_touch() {
            let mut l: LinkedListInner<i32> = LinkedListInner::new();
            l.push(0);
            assert_eq!(l.touch(), Some(&0));
        }
        #[test]
        fn test_linked_list_inner_touch_mut() {
            let mut l: LinkedListInner<i32> = LinkedListInner::new();
            l.push(0);
            *(l.touch_mut().unwrap()) = 1;
            assert_eq!(l.touch(), Some(&1));
        }
        #[test]
        fn test_linked_list_inner_is_empty() {
            assert!(LinkedListInner::<bool>::new().is_empty());
        }
        #[test]
        fn test_linked_list_inner_len() {
            assert_eq!(LinkedListInner::<bool>::new().len(), 0);
        }
    }
}

pub mod deque {
    use std::marker::PhantomData;
    use std::ptr::NonNull;
    // use std::sync::{Arc, Mutex};

    type Link<T> = Option<NonNull<DequeNode<T>>>;

    struct DequeNode<T> {
        front: Link<T>,
        back: Link<T>,
        data: T,
    }

    pub struct UnsafeDeque<T>(DoubleLinkListInner<T>);

    impl<T> UnsafeDeque<T> {
        #[inline]
        pub fn new() -> Self {
            Self(DoubleLinkListInner::new())
        }
        #[inline]
        pub fn push_front(&mut self, data: T) {
            self.0.push_front(data);
        }
        #[inline]
        pub fn push_back(&mut self, data: T) {
            self.0.push_back(data);
        }
        #[inline]
        pub fn pop_front(&mut self) -> Option<T> {
            self.0.pop_front()
        }
        #[inline]
        pub fn pop_back(&mut self) -> Option<T> {
            self.0.pop_back()
        }
        #[inline]
        pub fn front(&self) -> Option<&T> {
            self.0.front()
        }
        #[inline]
        pub fn front_mut(&mut self) -> Option<&mut T> {
            self.0.front_mut()
        }
        #[inline]
        pub fn back(&self) -> Option<&T> {
            self.0.back()
        }
        #[inline]
        pub fn back_mut(&mut self) -> Option<&mut T> {
            self.0.back_mut()
        }
        #[inline]
        pub fn len(&self) -> usize {
            self.0.len()
        }
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    /// Double-linked list
    ///
    struct DoubleLinkListInner<T> {
        front: Link<T>,
        back: Link<T>,
        len: usize,
        _marker: PhantomData<T>,
    }

    impl<T> Drop for DoubleLinkListInner<T> {
        fn drop(&mut self) {
            while self.pop_front().is_some() {}
        }
    }

    unsafe impl<T: Send> Send for DoubleLinkListInner<T> {}
    unsafe impl<T: Sync> Sync for DoubleLinkListInner<T> {}

    impl<T> DoubleLinkListInner<T> {
        pub fn new() -> Self {
            Self {
                front: None,
                back: None,
                len: 0,
                _marker: PhantomData,
            }
        }
        pub fn push_front(&mut self, data: T) {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(DequeNode {
                    front: None,
                    back: None,
                    data,
                })));
                if let Some(old) = self.front {
                    (*old.as_ptr()).front = Some(new);
                    (*new.as_ptr()).back = Some(old);
                } else {
                    self.back = Some(new);
                }
                self.front = Some(new);
                self.len += 1;
            }
        }
        pub fn push_back(&mut self, data: T) {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(DequeNode {
                    back: None,
                    front: None,
                    data,
                })));
                if let Some(old) = self.back {
                    (*old.as_ptr()).back = Some(new);
                    (*new.as_ptr()).front = Some(old);
                } else {
                    self.front = Some(new);
                }
                self.back = Some(new);
                self.len += 1;
            }
        }
        pub fn pop_front(&mut self) -> Option<T> {
            unsafe {
                self.front.map(|node| {
                    let boxed_node = Box::from_raw(node.as_ptr());
                    let result = boxed_node.data;

                    self.front = boxed_node.back;
                    if let Some(new) = self.front {
                        (*new.as_ptr()).front = None;
                    } else {
                        self.back = None;
                    }
                    self.len -= 1;
                    result
                })
            }
        }
        pub fn pop_back(&mut self) -> Option<T> {
            unsafe {
                self.back.map(|node| {
                    let boxed_node = Box::from_raw(node.as_ptr());
                    let result = boxed_node.data;

                    self.back = boxed_node.front;
                    if let Some(new) = self.back {
                        (*new.as_ptr()).back = None;
                    } else {
                        self.front = None;
                    }
                    self.len -= 1;
                    result
                })
            }
        }
        pub fn front(&self) -> Option<&T> {
            unsafe { self.front.map(|node| &(*node.as_ptr()).data) }
        }

        pub fn front_mut(&mut self) -> Option<&mut T> {
            unsafe { self.front.map(|node| &mut (*node.as_ptr()).data) }
        }

        pub fn back(&self) -> Option<&T> {
            unsafe { self.back.map(|node| &(*node.as_ptr()).data) }
        }

        pub fn back_mut(&mut self) -> Option<&mut T> {
            unsafe { self.back.map(|node| &mut (*node.as_ptr()).data) }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    #[cfg(test)]
    mod test_deque_inner {
        use super::DoubleLinkListInner;
        #[test]
        fn test_dequeue_inner_push_front_pop_back() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            (0..1000).into_iter().for_each(|value| {
                l.push_front(value);
            });
            (0..1000).into_iter().for_each(|value| {
                assert_eq!(l.pop_back(), Some(value));
            });
        }
        #[test]
        fn test_dequeue_inner_push_back_pop_front() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            (0..1000).into_iter().for_each(|value| {
                l.push_back(value);
            });
            (0..1000).into_iter().for_each(|value| {
                assert_eq!(l.pop_front(), Some(value));
            });
        }
        #[test]
        fn test_dequeue_inner_push_front_pop_front() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            (0..1000).into_iter().for_each(|value| {
                l.push_front(value);
            });
            (0..1000).into_iter().rev().for_each(|value| {
                assert_eq!(l.pop_front(), Some(value));
            });
        }
        #[test]
        fn test_dequeue_inner_push_back_pop_back() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            (0..1000).into_iter().for_each(|value| {
                l.push_back(value);
            });
            (0..1000).into_iter().rev().for_each(|value| {
                assert_eq!(l.pop_back(), Some(value));
            });
        }
        #[test]
        fn test_dequeue_inner_front() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            l.push_front(11);
            assert_eq!(l.front(), Some(&11_i32))
        }
        #[test]
        fn test_dequeue_inner_front_mut() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            l.push_front(11);
            *(l.front_mut().unwrap()) = 12;
            assert_eq!(l.front(), Some(&12_i32))
        }
        #[test]
        fn test_dequeue_inner_back() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            l.push_back(11);
            assert_eq!(l.back(), Some(&11_i32))
        }
        #[test]
        fn test_dequeue_inner_back_mut() {
            let mut l: DoubleLinkListInner<i32> = DoubleLinkListInner::new();
            l.push_back(11);
            *(l.back_mut().unwrap()) = 12;
            assert_eq!(l.back(), Some(&12_i32))
        }
        #[test]
        fn test_dequeue_inner_is_empty() {
            assert!(DoubleLinkListInner::<bool>::new().is_empty())
        }
        #[test]
        fn test_dequeue_inner_len() {
            let mut l = DoubleLinkListInner::<bool>::new();
            assert_eq!(l.len(), 0);
            l.push_back(false);
            assert_eq!(l.len(), 1);
        }
    }
}

#[cfg(test)]
mod test_stack {
    // use crate::list::UnsafeDeque;
    use super::stack::UnsafeStack;
    // use std::thread;
    // use std::time::Duration;

    #[test]
    fn test_stack_push_pop() {
        let mut l: UnsafeStack<i32> = UnsafeStack::new();
        (0..1000).into_iter().for_each(|value| {
            l.push(value);
        });
        (0..1000).into_iter().rev().for_each(|value| {
            assert_eq!(l.pop(), Some(value));
        });
    }
    #[test]
    fn test_stack_touch() {
        let mut l: UnsafeStack<i32> = UnsafeStack::new();
        l.push(0);
        assert_eq!(l.touch(), Some(&0));
    }
    #[test]
    fn test_stack_touch_mut() {
        let mut l: UnsafeStack<i32> = UnsafeStack::new();
        l.push(0);
        *(l.touch_mut().unwrap()) = 1;
        assert_eq!(l.touch(), Some(&1));
    }
    #[test]
    fn test_stack_is_empty() {
        assert!(UnsafeStack::<bool>::new().is_empty());
    }
    #[test]
    fn test_stack_len() {
        assert_eq!(UnsafeStack::<bool>::new().len(), 0);
    }
    // #[test]
    // fn test_thread_safe_linked_list() {
    //     let mut list: SafeStack<u64> = SafeStack::new();
    //     let list_ref = list.clone();
    //
    //     thread::scope(move |s| {
    //         let t1 = Duration::from_millis(11);
    //         let t2 = Duration::from_millis(23);
    //         s.spawn(move || {
    //             (0..100).into_iter().for_each(|x| {
    //                 list.push(x);
    //                 thread::sleep(t1);
    //             });
    //         });
    //         s.spawn(move || {
    //             (0..100).into_iter().for_each(move |_x| {
    //                 thread::sleep(t2);
    //                 match list_ref.pop() {
    //                     Some(val) => (), //println!("{}", val),
    //                     None => println!("None"),
    //                 }
    //             });
    //         });
    //     });
    // }
}

#[cfg(test)]
mod test_deque {
    use super::deque::UnsafeDeque;
    #[test]
    fn test_dequeue_push_front_pop_back() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        (0..1000).into_iter().for_each(|value| {
            l.push_front(value);
        });
        (0..1000).into_iter().for_each(|value| {
            assert_eq!(l.pop_back(), Some(value));
        });
    }
    #[test]
    fn test_dequeue_push_back_pop_front() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        (0..1000).into_iter().for_each(|value| {
            l.push_back(value);
        });
        (0..1000).into_iter().for_each(|value| {
            assert_eq!(l.pop_front(), Some(value));
        });
    }
    #[test]
    fn test_dequeue_push_front_pop_front() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        (0..1000).into_iter().for_each(|value| {
            l.push_front(value);
        });
        (0..1000).into_iter().rev().for_each(|value| {
            assert_eq!(l.pop_front(), Some(value));
        });
    }
    #[test]
    fn test_dequeue_push_back_pop_back() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        (0..1000).into_iter().for_each(|value| {
            l.push_back(value);
        });
        (0..1000).into_iter().rev().for_each(|value| {
            assert_eq!(l.pop_back(), Some(value));
        });
    }
    #[test]
    fn test_dequeue_front() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        l.push_front(11);
        assert_eq!(l.front(), Some(&11_i32))
    }
    #[test]
    fn test_dequeue_front_mut() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        l.push_front(11);
        *(l.front_mut().unwrap()) = 12;
        assert_eq!(l.front(), Some(&12_i32))
    }
    #[test]
    fn test_dequeue_back() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        l.push_back(11);
        assert_eq!(l.back(), Some(&11_i32))
    }
    #[test]
    fn test_dequeue_back_mut() {
        let mut l: UnsafeDeque<i32> = UnsafeDeque::new();
        l.push_back(11);
        *(l.back_mut().unwrap()) = 12;
        assert_eq!(l.back(), Some(&12_i32))
    }
    #[test]
    fn test_dequeue_is_empty() {
        assert!(UnsafeDeque::<bool>::new().is_empty())
    }
    #[test]
    fn test_dequeue_len() {
        let mut l = UnsafeDeque::<bool>::new();
        assert_eq!(l.len(), 0);
        l.push_back(false);
        assert_eq!(l.len(), 1);
    }
}
