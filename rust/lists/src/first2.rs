use std::mem;
/*
pub enum List {
    Empty,
    Elem(i32, List),
}
*/
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
	List { head: Link::Empty}
    }

    pub fn push(&mut self, elem:T) {
	let new_node = Box::new(Node {
	    elem: elem,
	    next: mem::replace(&mut self.head, Link::Empty),
	});

	self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
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
fn main() {

}
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = List::<&str>::new();
	assert_eq!(list.pop(), None);
	list.push("a");
	list.push("b");
	list.push("c");

	assert_eq!(list.pop(), Some("c"));
	assert_eq!(list.pop(), Some("b"));

	list.push("d");
	list.push("e");

	list.pop();
	list.pop();
	assert_eq!(list.pop(), Some("a"));
	assert_eq!(list.pop(), None);
    }
}
