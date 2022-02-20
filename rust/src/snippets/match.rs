fn main() {
    let x = 1;
    match x {
	1 | 2 => println!("One or two"),
	_ => println!("anything"),
    }

    match x {
	//1 ... 5 => {},  is depricated
	// ..= is now inclusive
	1 ..= 5 => println!("One through five"),
	_ => println!("something else"),
    }



    //match guard: additional condition
    let num = Some(4);
    match num {
	Some(x) if x < 5 => println!("less than five: {}", x),
	Some(x) => println!("{}", x),
	None => (),
    }

    //@ binding save varaible value while testing

    enum Message {
	Hello{id: i32},
    }
    
    let num = Message::Hello{id:5};
    match num {
	Message::Hello{id: id_var @ 3..=7} => {
	    println!("Found id = {}", id_var);
	},
	_ => (),
    }
}
