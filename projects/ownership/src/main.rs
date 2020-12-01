
fn main() {
    let mut y = " ";
    println!("Hello, world!");
    {
        let mut s = "hello";
        println!("y = {}, s= {}", y, s);
        s = "what is your name";
        y = s;
    }
    // s = "me";
    println!("{}", y);
    // println!("y = {}, s= {}", y, s);
    let mut somes = String::from("junk string 1");
    somes.push_str("this is real junk");
    ownership();
    let s3 = ownership1(somes);
    println!("main::s3 {}", s3);
}

fn ownership() {
    let mut s = String::from("what is your name? ");
    println!("{}", s);
    s.push_str(", Are you sure?");
    println!("{}", s);
    let s2 = s.clone();
    println!("{}", s); // will not compile as the s is no more considered valid
    println!("{}", s2);
    
}

fn ownership1(ss: String) -> String {
    println!("{}", ss);
    let mut ss1 = String::from("really junk string ");
    let ll = calculate_len(&mut ss1);
    println!("length of the string ss1 is {} and string is {}", ll, ss1);
    return ss;
}


fn calculate_len(s: & mut String) -> usize {
    s.push_str("asdasdasdads");
    return s.len();
}
