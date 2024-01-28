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
    fn push_and_pop() {
        let mut queue = PriorityQueue::new();
        assert_eq!(0, queue.len());

        queue.push(Elem::new(0));
        assert_eq!(1, queue.len());

        queue.push(Elem::new(1));
        assert_eq!(2, queue.len());

        let pop_node0 = queue.pop().unwrap();
        let pop_node1 = queue.pop().unwrap();

        assert!(Elem::new(0) == pop_node0);
        assert!(Elem::new(1) == pop_node1);
        assert_eq!(0, queue.len());
    }
}
