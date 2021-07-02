fn main() {
    // let s = String::from("hello");
    // let s1 = s;
    // println!("s1{}", s);
    let s2 = String::from("test");
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
    take_ownership(s2);
    let y = 5;
    makes_copy(y);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
