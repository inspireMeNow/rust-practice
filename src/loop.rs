fn another_function(x: i32, label: char) {
    println!("x = {x}, label = {label}");
}
//斐波那契数列
fn r#fn(x: i32) -> i32 {
    if x > 1 {
        r#fn(x - 1) + r#fn(x - 2)
    } else if x == 1 {
        1
    } else {
        0
    }
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
pub fn main() {
    println!("hello world!");
    another_function(5, 'g');
    //加上分号是语句，不会返回值
    let y = {
        let x = 3;
        x + 1
    };
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            //使用break返回值
            break counter * 2;
        }
    };
    println!("x is {}", five(3));
    println!("max is {}", cmp(3, 4).to_string());
    println!("result = {}, counter = {}", result, counter);
    //循环标签
    let mut count = 0;
    'counting_up: loop {
        println!("counting up {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                //只退出内层循环
                break;
            }
            if count == 2 {
                //退出外层循环
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    let mut number = 3;
    //while循环
    while number != 0 {
        println!("{number}");
        number = number - 1;
    }
    println!("LIFFTOFF!!!");
    //for循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the vaule is {element}");
    }
    //使用Range返回从一个数字开始到另一个数字之前结束的所有数字的序列
    for number in (1..5).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!("fib(10) = {}", r#fn(10));
}
