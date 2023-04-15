pub fn main() {
    let x = 5 + 1;
    {
        let x = x * 2;
        println!("x is {}", x);
    }
    println!("x is {}", x);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces);
    let y: u8 = 255;
    //类型转换
    let y1: u16 = y.into();
    let z = b'A';
    //整型
    // let f = 43 / 5;
    //强制类型转换
    let f: f32 = 43 as f32 / 5 as f32;
    let r#bool = true;
    let heart_eyed_cat: char = '😻';
    //元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "y is {0}, z is {1}, f is {2}, bool is {3}, emoji is {4}",
        y1 * 2,
        z,
        f,
        bool,
        heart_eyed_cat
    );
    //从元组中获取单个值
    // let (x, y, z) = tup;
    let x = tup.2;
    let y = tup.1;
    let z = tup.0;
    let tup1 = ();
    println!("x = {}, y = {}, z = {}", x, y, z);
    //数组类型
    let a: [i8; 5] = [1, 2, 3, 4, 5];
    //等价于let a = [3, 3, 3, 3, 3]
    let a1 = [3; 5];
    println!("{:p}", &a1);
    println!("Please input an array index.");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    //数组索引访问
    let index: usize = index.trim().parse().expect("Please input a number!");
    let element = a[index];
    println!(
        "The value of the element is: {} , index is {}",
        element, index
    );
}
