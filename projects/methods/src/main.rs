
#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}

// when the function inside impl takes self as parameter, it is called Method
// if withn the impl block function does not take self, it is called associated function
// Associated function is associated with the structure and not a specific instance of structure
impl Rectangle{
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold_rect(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
        (self.width >other.height && self.height > other.width) 
    }
    fn square(sz:u32)-> Rectangle{
        Rectangle {
            height:sz, 
            width:sz
        }
    }
}

#[derive(Debug)]
struct Circle{
    radius: f64
}

// Each struct is allowed multiple impl block as shown below for Circle
impl Circle{
    fn area(&self) -> f64 {
        self.radius * self.radius * (22.0/7.0)
    }
}

impl Circle{
    fn perimeter(&self) -> f64 {
        2.0 * self.radius * 22.0/7.0
    }
}

fn main() {
    println!("Hello, Rust Methods!");

    let rect1 = Rectangle {height: 20, width:30};
    let rect2 = Rectangle {height: 19, width:29};
    let rect3 = Rectangle {height: 39, width:19};
    println!("Area of rectangle {:?} is {}",rect1, rect1.area());
    let cir1 = Circle{radius: 22.0};
    println!("Area of circle {:?} is {} and perimeter is {}", cir1, cir1.area(), cir1.perimeter());
    println!("{}", 22/7);
    println!("Can rect1 hold rect2 {}", rect1.can_hold_rect(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold_rect(&rect3));

    println!("Square id {:#?}", Rectangle::square(10));
}

