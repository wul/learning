#[warn(unused_imports)]
use std::mem;    
use std::ops::Deref;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
	let mut cur_link = mem::replace(&mut self.head, None);
	while let Some(mut boxed_node) = cur_link {
	    cur_link = boxed_node.next.take();
	}
    }
}

//for list.into_iter()
pub struct IntoIter<T>(List<T>);

//for list.iter()
pub struct ListIter<'a, T> {
    cur: &'a Link<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
	self.cur.as_ref().map(|x| {
	    self.cur = &x.next;
	    &x.elem})
    }
}
    
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
	self.next.map(|node| {
	    self.next = node.next.as_deref();
	    &node.elem
	})
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
	List { head: None}
    }

    pub fn push(&mut self, elem:T) {
	self.head = Some(Box::new(Node {
	    elem,
	    next: self.head.take(),
	}));
    }


    pub fn peek(&self) -> Option<&T> {
	//Both OK

	/*
	 * head.as_ref(): Option<Box> -> Option<&Box> -> Option<&T>
	*/
	self.head.as_ref().map(|node| 
	    &node.elem
	)


	// head.as_deref(): Option<Box> -> Option<&Box> -> Option<&Box.deref()>
	// ->Option<&T>
	//self.head.as_deref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
	self.head.as_mut().map(|node| {
	    &mut node.elem
	})
    }

    pub fn pop(&mut self) -> Option<T> {
	self.head.take().map(|node| {
	    self.head = node.next;
	    node.elem
	})
    }
    
    pub fn into_iter(self) -> IntoIter<T> {
	IntoIter(self)
    }

    /*
    my own iterator
    pub fn iter<'a> (&'a self) -> ListIter<'a, T> {
	ListIter {cur: &self.head}
    }
    */
    pub fn iter<'a> (&'a self) -> Iter<'a, T> {
	//all works
	
	Iter {next: self.head.as_deref()}
	//Iter {next: self.head.as_ref().map::<&Node<T>,_>(|x| &x)}
	//Iter {next: self.head.as_ref().map(|x| x.deref())}
	//Iter {next: self.head.as_ref().map(|x| x.as_ref())}	
	//Iter {next: self.head.as_ref().map(|x| &**x)}	
    }

}


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
	self.0.pop()
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
    #[test]
    fn peek() {
	let mut list = List::new();
	assert_eq!(list.peek(), None);
	assert_eq!(list.peek_mut(), None);
	list.push(1); list.push(2); list.push(3);

	assert_eq!(list.peek(), Some(&3));
	assert_eq!(list.peek_mut(), Some(&mut 3));

	list.peek_mut().map(|value| {
	    *value = 4
	});

	assert_eq!(list.peek(), Some(&4));
	assert_eq!(list.pop(), Some(4));
	    
    }

    #[test]
    fn into_iter() {
	let mut list = List::new();
	list.push(1); list.push(2); list.push(3);
	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(3));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next(), None);


    }

    #[test]
    fn iter() {
	let mut list = List::new();
	list.push(1); list.push(2); list.push(3);
	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&3));
	assert_eq!(iter.next(), Some(&2));
	assert_eq!(iter.next(), Some(&1));
	assert_eq!(iter.next(), None);


    }    
}
