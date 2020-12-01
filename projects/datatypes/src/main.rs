fn main() {
    println!("Hello, world!");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 1.112, "abc");
    let (x,y,z) = tup;
    println!("x={}, y={}m z={} ",x,y,z);
    println!("{}, {}, {}", tup.1, tup.0, tup.2);
}
