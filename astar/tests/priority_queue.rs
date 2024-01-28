use std::cell::RefCell;
use astar::{Vector2, Node, PriorityQueue};

#[cfg(test)]
mod priority_queue_tests {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::rc::Rc;
    use astar::{Node, PriorityQueue, Vector2};

    struct Elem {
        v: i32
    }

    impl Elem {
        fn new(v: i32) -> Elem {
            Elem {
                v
            }
        }
    }

    impl Eq for Elem {
    }

    impl Ord for Elem {
        fn cmp(&self, other: &Self) -> Ordering {
            self.v.cmp(&other.v)
        }
    }

    impl PartialEq for Elem {
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v
        }
    }

    impl PartialOrd for Elem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            return if self == other {
                Some(Ordering::Equal)
            } else if self.v > other.v {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }

        fn lt(&self, other: &Self) -> bool {
            self.v < other.v
        }

        fn le(&self, other: &Self) -> bool {
            self.v <= other.v
        }

        fn gt(&self, other: &Self) -> bool {
            self.v > other.v
        }

        fn ge(&self, other: &Self) -> bool {
            self.v >= other.v
        }
    }

    #[test]
    fn push() {
        let mut q = PriorityQueue::new();

        q.push(1);
        assert_eq!(1, q.data()[0].unwrap());
        assert_eq!(1, q.len());

        q.push(3);
        assert_eq!(3, q.data()[0].unwrap());
        assert_eq!(1, q.data()[1].unwrap());
        assert_eq!(2, q.len());

        q.push(2);
        assert_eq!(3, q.data()[0].unwrap());
        assert_eq!(1, q.data()[1].unwrap());
        assert_eq!(2, q.data()[2].unwrap());
        assert_eq!(3, q.len());

        q.push(4);
        assert_eq!(4, q.data()[0].unwrap());
        assert_eq!(3, q.data()[1].unwrap());
        assert_eq!(2, q.data()[2].unwrap());
        assert_eq!(1, q.data()[3].unwrap());
        assert_eq!(4, q.len());

        q.push(0);
        assert_eq!(4, q.data()[0].unwrap());
        assert_eq!(3, q.data()[1].unwrap());
        assert_eq!(2, q.data()[2].unwrap());
        assert_eq!(1, q.data()[3].unwrap());
        assert_eq!(0, q.data()[4].unwrap());
        assert_eq!(5, q.len());
    }

    #[test]
    fn push_and_pop() {
        let mut q = PriorityQueue::new();

        q.push(0);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        assert_eq!(5, q.len());

        let four = q.pop();
        assert!(four.is_some());
        assert_eq!(4, four.unwrap());
        assert_eq!(4, q.len());

        let three = q.pop();
        assert!(three.is_some());
        assert_eq!(3, three.unwrap());
        assert_eq!(3, q.len());

        let two = q.pop();
        assert!(two.is_some());
        assert_eq!(2, two.unwrap());
        assert_eq!(2, q.len());

        let one = q.pop();
        assert!(one.is_some());
        assert_eq!(1, one.unwrap());
        assert_eq!(1, q.len());

        let zero = q.pop();
        assert!(zero.is_some());
        assert_eq!(0, zero.unwrap());
        assert_eq!(0, q.len());

        assert!(q.pop().is_none());
    }
}
