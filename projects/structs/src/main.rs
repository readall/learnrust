

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