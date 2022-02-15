
fn main() {

    //1. Does not work, moved
    let maybe_name = Some(String::from("Alice"));
    // The variable 'maybe_name' is consumed here ...
    match maybe_name {
	Some(n) => println!("Hello, {}", n),
	_ => println!("Hello, world"),
    }

    /*
    // ... and is now unavailable.
    println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
    */

    //2. Works, we use as_ref 
    let maybe_name = Some(String::from("Alice"));
    // The variable 'maybe_name' is consumed here ...
    match maybe_name.as_ref() {
	Some(n) => println!("Hello, {}", n),
	_ => println!("Hello, world"),
    }

    println!("Hello again, {}", maybe_name.unwrap_or("world".into()));


    //3. Works, we use ref keyword
    let maybe_name = Some(String::from("Alice"));
    // The variable 'maybe_name' is consumed here ...
    match maybe_name {
	Some(ref n) => println!("Hello, {}", n),
	_ => println!("Hello, world"),
    }

    println!("Hello again, {}", maybe_name.unwrap_or("world".into()));


    //
    // Option is a bit of different
    //
   
    //1. Work
    let option_name: Option<String> = Some("Alice".to_owned());
    match &option_name {
        Some(name) => println!("Hello, {}", name),
        None => println!("No name provided"),
    }

    //2. Work ref used
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name {
        Some(ref name) => println!("Hello, {}", name),
        None => println!("No name provided"),
    }
    println!("Hello again, {:?}", option_name);

    //3. Work as_ref used
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name.as_ref() {
        Some(name) => println!("Hello, {}", name),
        None => println!("No name provided"),
    }
    println!("Hello again, {:?}", option_name);    


    //3. Work as_deref used. It works just like as_ref, but additionally performs
    // a deref method call on the value
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name.as_deref() {
        Some(name) => println!("Hello, {}", name),
        None => println!("No name provided"),
    }
    println!("Hello again, {:?}", option_name);    
    
    //4. long ago version of 2
    let option_name: Option<String> = Some("Alice".to_owned());
    match &option_name {
        &Some(ref name) => println!("Hello, {}", name),
        &None => println!("No name provided"),
    }
    println!("Hello again, {:?}", option_name);
}
