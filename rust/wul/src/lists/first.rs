use std::mem;
/*
pub enum List {
    Empty,
    Elem(i32, List),
}
*/
#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
	List { head: Link::Empty}
    }

    pub fn push(&mut self, elem:i32) {
	let new_node = Box::new(Node {
	    elem: elem,
	    next: mem::replace(&mut self.head, Link::Empty),
	});

	self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
	let head = mem::replace(&mut self.head, Link::Empty);
	match head {
	    Link::Empty => {
		None
	    }
	    Link::More(node) => {
		self.head = node.next;
		Some(node.elem)
	    }
	}
	//unimplemented!()
    }
}

    
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = List::new();
	assert_eq!(list.pop(), None);
	list.push(1);
	list.push(2);
	list.push(3);

	assert_eq!(list.pop(), Some(3));
	assert_eq!(list.pop(), Some(2));

	list.push(4);
	list.push(5);

	list.pop();
	list.pop();
	assert_eq!(list.pop(), Some(1));
	assert_eq!(list.pop(), None);
    }
}
