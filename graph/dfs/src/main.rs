use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            edges: vec![]
        }
    }

    fn link_next(&mut self, next: &Rc<RefCell<Node>>) {
        self.edges.push(Rc::clone(next));
    }

    fn find_value_with_dfs(node: &Rc<RefCell<Node>>, value: i32) -> Option<Rc<RefCell<Node>>> {
        println!("node.value = {}", node.borrow().value);
        if node.borrow().value == value {
            return Some(node.clone());
        } else {
            for edge in node.borrow().edges.iter() {
                if let Some(found) = Node::find_value_with_dfs(edge, value) {
                    return Some(found);
                }
            }
        }
        None
    }
}

fn main() {
    let node_0 = Rc::new(RefCell::new(Node::new(0)));
    let node_1 = Rc::new(RefCell::new(Node::new(1)));
    let node_2 = Rc::new(RefCell::new(Node::new(2)));
    let node_3 = Rc::new(RefCell::new(Node::new(3)));
    let node_4 = Rc::new(RefCell::new(Node::new(4)));
    let node_5 = Rc::new(RefCell::new(Node::new(5)));

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

    if let Some(found) = Node::find_value_with_dfs(&node_0, 5) {
        println!("found.value = {}", found.borrow().value);
    } else {
        println!("not found.");
    }
}
