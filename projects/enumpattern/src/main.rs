
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) ->u8 {
    let mut count: u32 =0; // but this is inside a function so is reset every time
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            count += 1;
            println!("Coin is from state {:?}, {}", state, count);
            25
        },
    }
}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}


fn main() {
    println!("Hello, world enum and patterns!");
    let some_number = Some(5);
    let some_str = Some("a string");
    let abselt_number : Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_str, abselt_number);

    println!("{:?}",value_in_cents(Coin::Nickel));
    println!("{:?}",value_in_cents(Coin::Penny));
    println!("{:?}",value_in_cents(Coin::Dime));
    println!("{:?}",value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?}",value_in_cents(Coin::Quarter(UsState::Alaska)));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    let somevalue = 2u8;
    match somevalue {
        1 => println!("One"),
        2 => println!("two"),
        6 => println!("six"),
        _ => (),
    }

    let somevalue = 3u8;
    match somevalue {
        1 => println!("One"),
        2 => println!("two"),
        6 => println!("six"),
        _ => (),
    }

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("Three with if let");
    }
}
