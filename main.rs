struct Node {
    next: Option<Box<Node>>,
    data: String,    
}

use std::fmt;

impl fmt::Pointer for Option<Box<Node>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "there should be the address here :'(")
    }
}

fn main() {

    let newNode: Option<Box<Node>> = Some(Box::new(Node {next: None, data: "Hello".to_string()}));

    let secondNode: Node = Node {next: newNode, data: "some data".to_string()};

    println!("{:p}, {}", secondNode.next, secondNode.data);

}
