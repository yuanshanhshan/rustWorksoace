fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is:{}", y);
    println!("The value of x is:{}", x);
    println!("The value of z is:{}", z);
    println!("tup2:{}", tup2.0);
    let array_a = [1, 2, 3, 4, 5];
    println!("The value of x is:{}", array_a[0]);
    another_function(32, 33);

    let x = five();
    println!("The value of x is:{}", x);

    let y = plus_one(100);
    println!("plus_one():{}", y);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.The value of x is:{}:", x);
    println!("Another function.The value of x is:{}:", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


