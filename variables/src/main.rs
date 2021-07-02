fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // add
    let sum = 5 + 10;
    println!("sum {}", sum);

    // sub
    let sub = 95.5 - 2.2;
    println!("sub {}", sub);
    //mul
    let mul = 4 * 30;
    println!("mul {}", mul);
    // div
    let div = 10 / 3;
    println!("div {}", div);
    // mod
    let remainder = 43 % 5;
    println!("remainder {}", remainder);
}
