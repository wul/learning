fn main() {
    let x = Box::new(5);
    if *x == 5 {
	println!("* Box(5) == 5");
    }
    /*
    if x == 5 {
	println!("Box(5) == 5");
    }
*/

}
