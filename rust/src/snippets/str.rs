
fn main() {
    let s = "abc";
    let s1 = s;
    let s2 = s;
    println!("{:p} {:p}  {:p}", s, s1, s2);

    let ss = "abc".to_string();
    let ss1 = ss.clone();
    let ss2 = ss.clone();
    println!("{:p} {:p}  {:p}", &ss, &ss1, &ss2);

    let a:i32 = 1;
    let a1 = a;
    let a2 = a;
    println!("{:p} {:p}  {:p}", &a, &a1, &a2);

    let s = "a b  c  d";
    println!("s pointer is {:p}", s);
    for x in s.split_whitespace(){
	println!("{} {:p}", x, x);
    }
    
}
