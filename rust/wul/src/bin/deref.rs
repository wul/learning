use std::ops::Deref;

struct MyBox<T> {
    val: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
	&self.val
    }
}


fn main() {
    /* should not work 
     * Cannot compare a variable with it's reference
     */

    /*
    let a = 5;
    let b = &a;
    assert_eq!(a, b);
     */

    /* Cannot compare a variable with a Box pointer */
    /*
    let a = Box::new(5);
    assert_eq!(a, 5);
     */
    let a = Box::new(5);
    assert_eq!(*a, 5);


    let bx = MyBox {val:5};
    assert_eq!(*bx, 5);



    //Scenario 1: operator *
    let mut cx = MyBox {val: "abc".to_string()};
    assert_eq!(*cx, "abc".to_string());

    //Scenario 2: use T's immutable method
    println!("{}", cx.len());

    //Scenario 3: &MyBox equals &T

    fn test(s:&String) {
	println!("{:?}", s);
    }
    let val = MyBox{val: "abc".to_string()};
    test(&val);
    
}

    
