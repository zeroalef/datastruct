mod list;

fn main() {
    let mut l: LinkedListInner<i32> = LinkedListInner::new();
    (0..100).into_iter().for_each(|value| {
        l.cons(value);
    });
    (0..100).into_iter().rev().for_each(|value| {
        assert_eq!(l.car(), Some(value));
    });
}
