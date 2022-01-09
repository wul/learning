
//
// Single borrow with original one
//


/* mutable object cannot change while it's immutable refernce exists */
fn test1() -> u32{
    let mut x = 42;
    let y = &x;
    x += 1;

    //x已经重新接管变量内容，y失效
    //y
    11
}




//可变借用修改了内容，则原变量即使是可变的，也不能再修改
/* not works: y is stiall aliave, so x need to froze  */
fn test11() ->u32 {
    let mut x = 42;
    let y = &mut x;
    *y = 43;

    // x不能再修改
    //x += 1;

    *y = 45;

    //但是x可以读
    x
	
}






//
// 两个引用
//

fn test2() {

    let mut x = 42;
    let y = &mut x;
    let z = & x;


    //这里只可打印z，而不能打印y, 防止z的值被变动？
    println!("{}", z);
    //println!("{}", y);
    

}

fn main() {
    test1();
    test11();

    
    test2();

    


}
