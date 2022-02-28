
fn main() {
    let a = Some("abc".to_string());

    let b = a.as_ref().map(|x| x);
    println!("b is {:?}", b);

}
