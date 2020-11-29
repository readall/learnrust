fn main() {
    println!("Hello, world!");

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("Condition was false");
    }

    if number !=0 {
        println!("Number was other than zero");
    }

    let number = 8;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("Number is not divisible by 4,3 or 2")
    }

    let condition = false;
    let n1 = if condition { 5 } else {6};
    println!("{}", n1);
    // let n2: u16 = 22;
    // let n3: u32 = 55;
    // let n4 = n3 + n2;  // this will not complile as rust does not support implicit type conversions and promotions
    // println!("{}", n4) 
    loop_s();
    loop_array();
}


fn loop_s() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("resultant result is {}", result);

    while counter != 0 {
        println!("number = {}", counter);
        counter -= 1;

    }

}

fn loop_array(){

    let a = [10,20,30,40,50];
    let mut index =0;

    while index < a.len() {
        println!("{}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("Value of element is {}", element);
    }

    for number in (1..5).rev(){
        println!("{}", number);
    }
    for number in 1..5 {
        println!("{}", number);
    }
}
