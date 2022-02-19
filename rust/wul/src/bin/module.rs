mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

//works fine
//use self::front_of_house::hosting;


//works fine
//use crate::front_of_house::hosting;


//works fine
use front_of_house::hosting;

pub fn eat_at_restaurant () {
    hosting::add_to_waitlist ();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
