use std::collections::HashSet;

fn main() {
    let mut set = HashSet::<String>::new();

    set.insert("a".to_string());
    set.insert("b".to_string());
    set.insert("c".to_string());    

    let mut set2 = HashSet::<String>::new();

    set.iter().map(|x| set2.insert(x.clone()));
    println!("set.iter().map result {:?}", set2);

    set.iter().for_each(|x| {set2.insert(x.clone());});
    println!("set.iter().for_each result {:?}", set2);    

    set2 = set.iter().map(|x| x.clone()).collect::<HashSet::<String>>();
    println!("set.iter().map().collect() {:?}", set2);
    
    set2 = set.iter().map(Clone::clone).collect::<HashSet<String>>();
    println!("set.iter.map(Clone::clone) result {:?}", set2);

    
    set.into_iter().map(|x| set2.insert(x.clone()));
    println!("set.into_iter().map result {:?}", set2);    

    


    
}
	       
