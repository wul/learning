use std::hash::{Hash, Hasher};


// 
// PartialEq, Self vs self
// Self is type, self is instance
//

struct Employee {
  name: String,
  next_meeting: String,
}

impl PartialEq for Employee {
  fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


// Hash traits example
// 如果struct内所有属性都是可hash的，可以直接用宏
//
#[derive(Hash)]
struct Man {
    name: String,
    country: String,
}

// 或者自己实现Hash traits
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}

//to enable print Kind with {:?}
#[derive(Debug)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::A }
}



fn about_hasher () {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

    hasher.write(&data);
    let r = hasher.finish();
    println!("Hash is {:x}!", r);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let x:Kind = Default::default();
        /* Cannot compare two enum varaible, you have to use matches! macro or 
         * implement PartialEq traits
         *
         * #[derive(PartialEq)]
         * enum MyEnum { ... }
         */
         
        //assert_eq!(x, Kind::A);
        matches!(x, Kind::A);
    }
    #[test]
    fn test_default2() {
        let x:Kind = Kind::default();
        matches!(x, Kind::A);
    }
    #[test]
    fn test_about_hasher() {
        about_hasher();
        assert!(true);
    }
    
}



