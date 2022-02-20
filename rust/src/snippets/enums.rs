
enum IPAddr {
    V4( String),
    V6( String),
}


fn main() {
    let a = Option::<i32>::Some(5);
    let b = Option::<i32>::None;

    let c = Some::<i32>(6);
    println!("{:?}\n{:?}\n{:?}", a, b, c);
}


