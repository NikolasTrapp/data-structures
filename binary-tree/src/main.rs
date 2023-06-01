use std::fmt;

struct Node {
    value: i32,
    rigth: Option<Box<Node>>,
    left: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            rigth: None,
            left: None
        }
    }

    fn insert(&mut self, number: i32) {
        if number < self.value {
            if let Some(left) = &mut self.left {
                left.insert(number);
            } else {
                self.left = Some(Box::new(Node::new(number)));
            }
        } else {
            if let Some(rigth) = &mut self.rigth {
                rigth.insert(number);
            } else {
                self.rigth = Some(Box::new(Node::new(number)));
            }
        }
    }

    fn print(&mut self) {
        if let Some(next) = &mut self.left {
            next.print();
        }
        println!("{}", self.value);
        if let Some(next) = &mut self.rigth {
            next.print();
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let mut root = Node::new(50);

    root.insert(49);
    root.insert(45);
    root.insert(47);
    root.insert(53);
    root.insert(43);
    root.insert(40);
    root.insert(44);
    root.insert(46);
    root.insert(48);

    root.print();

}
