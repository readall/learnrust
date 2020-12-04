

fn main() {
    println!("Hello, world!");
    let user1 = User{
        email: String::from("som@some.com"),
        username: String::from("some one"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    let u1 = build_user("junk@junk.com", "punk at junk");
    println!("{}, {}, {}, {}",u1.sign_in_count, u1.email, u1.active, u1.username);
    let unam = String::from("username junk");
    let uem = String::from("user@email.com");
    let u1 = build_user_1(uem, unam);
    println!("{}, {}, {}, {}",u1.sign_in_count, u1.email, u1.active, u1.username);
    let u2 = build_user_frm_user(u1);
    println!("{}, {}, {}, {}",u2.sign_in_count, u2.email, u2.active, u2.username);
    tuple_structs();
    play_struct();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,

}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count:1,
    }
}

fn build_user_1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count:1,
    }
}

// fn build_user_frm_user(u1: &User) -> User {
//     let user2 = User {
//         email: String::from(&u1.email),
//         username: String::from(&u1.username),
//         active: u1.active,
//         sign_in_count: u1.sign_in_count
//     };
//     return user2;
// }


// this builds the new structure as copy of passed one
fn build_user_frm_user(u1: User) -> User {
    User {
        ..u1
    }
}

fn tuple_structs(){
    struct Color (i32, i32, i32);
    struct Point (i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);
    // black = origin; // does not compile as black and origin are of different types
                        // each struct definition is a new type and one type can not 
                        // be assigned to aoother
}

#[derive(Debug)] // this is like deriving from a debug class that implements serdes
struct Rectangle {
    width: u32,
    height: u32,
}

// this is tying the area function to Rectangle structure
// the area function must declare reference to self as first argument
// if &self is not declared, we can not access the particular instance
// &self here is immutable so we can not change width or length
// with "&mut self" the self becomes mutable
impl Rectangle {
    fn area(&mut self) -> u32 {
        self.height = 0;
        self.height * self.width
    }
}

fn play_struct() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    println!("Area of rectangle is {}", rect1.area());
    println!("rect1 is {:#?}", rect1);
}
