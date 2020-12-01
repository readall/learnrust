
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
    refer();
    let reference_to_nothing = dangle();
    println!("first_word:: {}",first_word(&reference_to_nothing));
    first_word_v1(&reference_to_nothing);

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

fn refer(){
    let mut s = String::from("mutable string s");
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
    let r3 = &mut s; // this code compiles because the immutable reference r1 and r2 are not used after this line
    r3.push_str(" really mutable reference to string");
    println!("{}", r3);
}

fn dangle() -> String {
    let s = String::from("hello fool");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // full string
}

// this function will cause panic in main when executed. 
// compiles fine with no error
// reason for suc a behavior is that we are trying to
// access an index from Sring that is no more valid and 
// that happens because call to ss.clear()
// In effect invalid index or array out of bounds kind of situation
fn first_word_v1(s: &String){
    let mut ss = String::from(s);
    let word = first_word(&ss);
    println!("{}, {}", ss, word);
    ss.clear();
    // println!("{}, {}", &ss[0..word], word);
    s_slice();
}

fn s_slice(){
    let s = String::from("Hello fool");
    let hello = &s[0..5];
    let fool = &s[6..10];
    println!("{} {}", hello, fool);
}
