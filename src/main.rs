struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn contains(&self, value: i32) -> bool {
        if self.value == value {
            true
        } else if value < self.value {
            if let Some(left) = &self.left {
                left.contains(value)
            } else {
                false
            }
        } else {
            if let Some(right) = &self.right {
                right.contains(value)
            } else {
                false
            }
        }
    }

    fn print(&self, depth: u32) {
        if let Some(right) = &self.right {
            right.print(depth + 1);
        }
        println!("{:indent$}{}", "", self.value, indent = (depth * 2) as usize);
        if let Some(left) = &self.left {
            left.print(depth + 1);
        }
    }
}

fn main() {
    let mut root = Node::new(5);
    root.insert(2);
    root.insert(5);
    root.insert(1);
    root.insert(6);
    root.insert(7);
    root.insert(10);

    root.print(0);

    println!("Contains 7: {}", root.contains(7));
    println!("Contains 2: {}", root.contains(2));
}
