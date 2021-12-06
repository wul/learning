

//Format 1
struct Point {
    x: i32,
    y: i32,
}

// Following struct are same

struct Foo1;
struct Foo2();
struct Foo3 {}       //no semi-colon


// Tuple struct

struct Color (i32, i32, i32);

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_struct1 () {
	    let p = Point {x: 1, y:-1};
	    assert_eq!(p.x, 1);
    }
    
    
    #[test]
    fn test_struct2 () {
        let x = 1;
        let y = 2;
        //
        // Point.x is just use variable x, same to the y
        //
	    let p = Point {x, y};
	    assert_eq!(p.x, 1);
    }
    
        #[test]
    fn test_struct3 () {
        let c = Color(0, 0, 0);
	    assert_eq!(c.2, 0);
    }
}

