#![allow(unused)]
mod r#const;
mod data;
mod game;
fn another_function(x: i32, label: char) {
    println!("x = {x}, label = {label}");
}
//函数返回值，if后面的表达式的值必须为bool类型
fn five(x: i32) -> i32 {
    let number = 3;
    if number > x {
        number
    } else if number == x {
        x + 1
    } else {
        x
    }
}
fn r#cmp(a: i32, b: i32) -> i32 {
    let condition = a > b;
    if condition {
        a
    } else {
        b
    }
}
fn main() {
    println!("hello world!");
    another_function(5, 'g');
    //加上分号是语句，不会返回值
    let y = {
        let x = 3;
        x + 1
    };
    println!("x is {}", five(3));
    println!(" max is {}", cmp(3, 4).to_string());
}
