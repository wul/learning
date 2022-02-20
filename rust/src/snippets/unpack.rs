struct Point {
    x: i32,
    y: i32,
    z: i32, 
}

fn main() {
    let var = (1,2,3);
    let (x,y,z) = var;

    
    let (x, ..) = var;

    let p = Point {x:0, y:7, z:8};
    let Point{x: a, y:b , z:c} = p;

    assert_eq!(0, a);
    assert_eq!(7,b);

    let Point{x, y, z} = p;

    assert_eq!(0, x);
    assert_eq!(7,y);


    //.. means exclusive
    match p {
	Point{x, ..} => println!("x is {}", x),
	_ => {},
    }
    
}
