
fn main () {
    let x = 5;
    let y = Box::new(x);

    let z = *y + 3;
    println!("{:?}", z);
}
