
//结构中所有的东西都是hashable的，直接继承traits即可
#[derive (PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    s: String,
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash () {
        let mut map: HashMap<Point, i32> = HashMap::new();
        let point1 = Point {
            x: 1,
            y: 2,
            s: "abc".to_string(),
        };
        let point2 = Point {
            x: 1,
            y: 2,
            s: "abc".to_string(),
        };
        map.insert(point1, -1);
        
        if let Some(&x) = map.get(&point2) {
            assert_eq!(x, -1);
        } else {
            assert_eq!(1,1);
        }
    }

    
}
