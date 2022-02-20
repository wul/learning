#[derive (Clone, Copy)]
struct MyStruct<'a> {
    //s: String,           //not implemented copy
    s2: &'a str,         //str does not implement copy , but &str does
}


fn main() {

}
