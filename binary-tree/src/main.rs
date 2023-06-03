use std::fmt;
use std::fmt::Debug;

struct Node {
    value: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            right: None,
            left: None
        }
    }

    fn print(&mut self) {
        if let Some(next) = &mut self.left {
            next.print();
        }
        println!("{}", self.value);
        if let Some(next) = &mut self.right {
            next.print();
        }
    }

    fn get_depth(&mut self, number: i32) -> u32 {
        let mut count = 0;
        if number == self.value {
            return 0
        }

        if number < self.value {
            if let Some(node) = &mut self.left {
                count = node.get_depth(number);
                return count + 1
            }
        } else {
            if let Some(node) = &mut self.right {
                count = node.get_depth(number);
                return count + 1
            }
        }
        return count
    }

    fn insert(&mut self, number: i32) {
        if self.value == number {
            return
        }
        if number < self.value {
            if let Some(node) = &mut self.left {
                node.insert(number);
            } else {
                self.left = Some(Box::new(Node::new(number)));
            }
        } else {
            if let Some(node) = &mut self.right {
                node.insert(number);
            } else {
                self.right = Some(Box::new(Node::new(number)));
            }
        }
    }

    fn delete(&mut self, number: i32) -> Option<Box<Node>>{
        if self.value < number {
            if let Some(node) = &mut self.left {
                node.delete(number);
            }
        } else if self.value > number {
            if let Some(node) = &mut self.right {
                node.delete(number);
            }
        } else {
            if self.left.is_none() {
                return self.right.take()
            } else if self.right.is_none() {
                return self.left.take()
            } else {
                let mut temp = self.right.as_mut().unwrap();
                while let Some(node) = &mut temp.left {
                    temp = node;
                }
                self.value = temp.value;
                self.right = self.right.take().and_then(|mut r| {
                    r.delete(temp.value)
                });

            }
        }
        Some(Box::new(self.clone()))
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

    root.delete(40);
    root.print();

}
