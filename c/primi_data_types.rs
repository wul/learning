
fn slice() {
   let arr = [1,2,3,4,5];
   //切片无法直接使用，必须通过引用
   //let slice1 = arr[0..3];
    let slice1 = &arr[0..3];


    //数组类型直接转换成slice类型
    //&[T;n] 当做slice来使用
    //因为.操作符的自动创建引用和解除引用，因此Array可以直接调用Slice的所有方法
    let arr = [1,2,3,4];
    let slice = &arr;
    printlnln!("{}", slice.first().unwrap());


    let arr = [10,20,30,40];
    println!("{}", arr.starts_with(&[10, 20]));
}

fn main() {
  //
  // Tuple & ()
  //

    let t = (1,2,3);
    let (x,y,z) = t;
    let v = t.1;

    //unit type
    let h: () = ();

    let n = 3;
    // n is not allowed here
    //let arr:[&str; n] = ["a", "b", "c"];
    let arr:[&str; 3] = ["a", "b", "c"];

    for i in &arr.iter() {
        println!("{}", i);
    }
}
