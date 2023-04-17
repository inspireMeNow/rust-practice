//加上mut关键字，使变量可变
fn calclute_length(s: &mut String) -> usize {
    //和变量默认不可变一样
    s.push_str(", world");
    s.len()
} //s的引用被传递到函数中，但是并没有所有权，所以函数执行完后，s仍然有效
  //悬垂引用
fn dangle() -> String {
    let s = String::from("hello");
    s //直接返回变量
} //s被丢弃，内存被释放，但是引用仍然指向这块内存，所以会报错
pub fn main() {
    let mut s1: String = String::from("hello");
    let len = calclute_length(&mut s1);
    println!("The length of {} is {}.", s1, len);
    //不能同时存在多个可变引用，避免数据竞争
    // let s2 = &mut s1;
    {
        let s2 = &mut s1;
        println!("s2 = {s2}");
    }
    let s2 = &s1;
    let s4 = &s1;
    println!("s2 = {s2}, s4 = {s4}");
    //此位置后s2、s4不再使用，所以可以继续使用s1
    let s3 = &mut s1;
    println!("s3 = {}", s3);
    let reference_to_nothing = dangle();
}
