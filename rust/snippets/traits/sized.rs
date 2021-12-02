/*
Sized trait
Sized 在Rust中是一个比较特殊的 trait ，该Sized trait默认是自动实现的。一个类型是否是Sized，要看它在编译期内的size是否已知且固定不变。比如，u8 的大小是 1 byte。

有些类型在编译期无法确定大小。

一个 slice的[T]的size是未知的，因为在编译期不知道到底会有多少个T存在。

一个trait的size是未知的，因为不知道实现这个trait的结构是什么。

把unsized的类型放到指针或者Box里面，就变成了sized了，通过指针找到源头，然后顺着源头找到其他的数据。

所有的类型参数，如fn foo<T>(){}中的T，默认都是实现了Sized的了（自动实现），这就限制了传参数的数据了，

?Sized trait
?Sized就表示UnSized类型。
*/

//这个函数可以接受unsized 类型的参数
fn foo<T:?Sized>(param: T)
{
}
