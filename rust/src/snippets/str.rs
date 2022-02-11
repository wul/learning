
fn main() {
    let s = "abc";
    let s1 = s;
    let s2 = s;
    println!("{:p} {:p}  {:p}", s, s1, s2);

    let ss = "abc".to_string();
    let ss1 = ss.clone();
    let ss2 = ss.clone();
    println!("{:p} {:p}  {:p}", &ss, &ss1, &ss2);    
}
