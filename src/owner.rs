fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calclute_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
pub fn main() {
    // r#loop::main();
    let mut s = String::from("hello");
    s.push_str(", world!");
    //移动s到s1中
    //let s1 = s;
    //克隆s到s1中
    let s1 = s.clone();
    //对整型来说，是拷贝
    //实现了 Copy trait的变量有：整型、浮点型、布尔型、字符型、元组（当且仅当其元素类型也都实现了 Copy trait）
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    let s = String::from("hello");
    //s的值移动到函数里，函数执行完后s不再有效
    takes_ownership(s);
    // println!("{}",s);
    //x是实现了 Copy trait的变量，所以后面可继续使用x
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    //s2已经被移动到takes_and_gives_back函数中，所以后面不能再使用s2
    // println!("{}",s2);
    println!("s1 = {}, s3 = {}", s1, s3);
    let s1 = String::from("hello");
    let (s1, len) = calclute_length(s1);
    println!("s1 = {}, length = {}", s1, len);
}