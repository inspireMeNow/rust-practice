#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
pub fn main() {
    //常量
    println!("Guess a number:");
    //rand生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input the guess:");
        let mut guess = String::new();
        //读取用户输入结果为Result类型，需要加上except方法传递错误信息
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
