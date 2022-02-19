
fn borrow_that(v: &Vec<i32>) -> &i32{
    return &v[0];
}

fn main()
{
    let mut v = vec![1,2,3];
    // Dont work
    //let first = &v[0];

    // Dont work neither
    let first = borrow_that(&v);
    v.push(4);

    //cannot uncomment next line. 会造成 v.push(4) 与 let first = &v[0]; 冲突
    //println!("The first element is: {:?}", first);
}
	
