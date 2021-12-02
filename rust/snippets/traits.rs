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

#[derive(Debug)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::A }
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
}
