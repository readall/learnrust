fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(44, -33);

}

// fn another_function() {
//     println!("Another function");
// }


fn another_function(a: u32, b: i32) {
    println!("Another function a={}, b={}", a, b);
    let x: f64 = 6.6 + 5.0;
    println!("{:?}",println!("{}", x));
    let mut y = five();
    println!("{}", y);
    y = plus_one(y);
    println!("{}", y);

    let x1 = 4 + -5;
    let x2 = 5 + x1;
    println!("{}", x1);
    println!("{}", x2);
}

fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32 {
    x+1
}