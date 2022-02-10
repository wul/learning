use std::cell::RefCell;
use std::rc::Rc;
fn main () {
    let x = 5;
    let y = Rc::new(RefCell::new(x));
    let a = y.borrow();
    let mut z = y.borrow_mut();
    
    *z = 6;

    println!("{}", x);
    println!("{:?}", a);
}
