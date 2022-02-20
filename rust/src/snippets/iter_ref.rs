fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    // work
    //for &item in list.iter() {
    //	if item > largest {
    //	    largest = item;    

    // work too
    for item in list.iter() {
    	if *item > largest {
    	    largest = *item;
	}
    }
    largest
}


fn main () {
    let number_list = vec![34,50,32,33,12];
    let result = largest(&number_list);
    println!("largetst is {}", result);

    let s_list = vec!["ab", "cd", "ef"];
    //why str has no copy trait, but still can be used here?
    //because all &T implmeneted Copy traits
    let result = largest(&s_list);
    println!("larget is {}", result);
}
