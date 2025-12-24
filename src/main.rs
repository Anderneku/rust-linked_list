use std::ptr::copy;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, next: Option<Box<Node>>) -> Self {
        Self { value, next }
    }
    fn set_next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            size: 0_i32,
        }
    }
    fn prepend(&mut self, mut node: Node) {
        node.next = self.head.take();
        self.head = Some(Box::new(node));
        self.size += 1;
    }
    fn append(&mut self, node: Node) {
        let mut current_node: &mut Option<Box<Node>> = &mut self.head;
        while let Some(node) = current_node {
            current_node = &mut node.next
        }
        *current_node = Some(Box::new(node));
        self.size += 1;
    }
    fn reverse(&mut self) {
        let mut current_node = self.head.take();
        let mut previous_node: Option<Box<Node>> = None;

        while let Some(mut node) = current_node {
            let next = node.next.take();
            node.next = previous_node;
            previous_node = Some(node);
            current_node = next;
        }
        self.head = previous_node; 
    }
    // Get_index, Insert, Is_empty, Reverse
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list: String = String::new();
        let mut current = &self.head;
        while let Some(node) = current {
            list.push_str(node.value.to_string().as_str()); // write!(list, "{}", node.value).unwrap()
            current = &node.next;
            if !node.next.is_none() {
                list.push_str("->")
            };
        }
        write!(f, "{}", list)
    }
}

fn main() {
    let mut list = LinkedList::new();
    let node1 = Node::new(34, None);
    let node2 = Node::new(64, None);
    list.prepend(node1);
    list.prepend(node2);
    list.append(Node::new(21, None));
    println!("{}", list.size);
    println!("{}", list);
    list.reverse();
    println!("{}", list);
}
