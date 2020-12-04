
#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    println!("Hello, Rust Methods!");

    let rect1 = Rectangle {height: 20, width:30};
    println!("Area of rectangle is {}", rect1.area());
}

