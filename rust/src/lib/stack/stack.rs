#[derive(Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl <T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
	Stack {
            top: None,
	}
    }
    fn push(&mut self, val:T) {
	let mut node = StackNode::new(val);
	let next = self.top.take();
	node.next = next;
	self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
	let val = self.top.take();
	match val {
	    None => None,
	    Some(mut x) => {
		self.top = x.next.take();
		Some(x.val)
	    },
	}
    }

    fn gettop(&self) -> Option<&T> {
	if let Some(ref v) = self.top {
	    return Some(&v.val);
	}
	return None;


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stack() {
	let mut stack = Stack::<u32>::new();
	assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack() {
	//Copy enable user3 = user1
	#[derive(Eq, PartialEq, Debug, Copy, Clone)]
	struct TestStruct <'a>{
	    name: &'a str,
	}
	let user1 = TestStruct{name: "Zhang San"};
	let user2 = TestStruct{name: "Li Si"};
	let user3 = user1;
	let mut stack = Stack::<TestStruct>::new();
	stack.push(user1);
	assert_eq!(stack.gettop(), Some(&user3));
    }
    #[test]
    fn test_stack_pop() {
	//Copy enable user3 = user1
	#[derive(Eq, PartialEq, Debug)]
	struct TestStruct{
	    name: String,
	}
	let user1 = TestStruct{name: "Zhang San".to_string()};
	let user2 = TestStruct{name: "Li Si".to_string()};

	let user3 = TestStruct{name: "Li Si".to_string()};
	let user1copy = TestStruct {name: "Zhang San".to_string()};
	
	let mut stack = Stack::<TestStruct>::new();
	stack.push(user1);
	stack.push(user2);
	assert_eq!(stack.gettop(), Some(&user3));
	let user = stack.pop();
	assert_eq!(user, Some(user3));
	assert_eq!(stack.gettop(), Some(&user1copy));
	stack.pop();
	assert_eq!(stack.gettop(), None);
    }
}
	    
      

	    
      
