use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Node {
    value: i32,
    edges: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            edges: vec![]
        }
    }

    fn rc_new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(value)))
    }

    fn link_next(&mut self, node: &Rc<RefCell<Node>>) {
        self.edges.push(Rc::clone(node));
    }

    fn find_value_with_bfs(starting_node: &Rc<RefCell<Node>>, value: i32) -> Option<Rc<RefCell<Node>>> {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(starting_node));

        while let Some(front) = q.pop_front() {
            let front_node = &front.borrow();
            if front_node.value == value {
                return Some(Rc::clone(&front));
            } else {
                for edge in &front_node.edges {
                    q.push_back(Rc::clone(edge));
                }
            }
        }
        None
    }
}

fn main() {
    let node_0 = Node::rc_new(0);
    let node_1 = Node::rc_new(1);
    let node_2 = Node::rc_new(2);
    let node_3 = Node::rc_new(3);
    let node_4 = Node::rc_new(4);
    let node_5 = Node::rc_new(5);

    // 0 - 1 - 3 - 5
    //   \ | / |
    //     2 - 4

    node_0.borrow_mut().link_next(&node_1);
    node_0.borrow_mut().link_next(&node_2);
    node_1.borrow_mut().link_next(&node_2);
    node_1.borrow_mut().link_next(&node_3);
    node_2.borrow_mut().link_next(&node_3);
    node_2.borrow_mut().link_next(&node_4);
    node_3.borrow_mut().link_next(&node_5);

    if let Some(found) = Node::find_value_with_bfs(&node_0, 5) {
        println!("found.value = {}", found.borrow().value);
    } else {
        println!("not found.");
    }
}
