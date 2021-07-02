// control flow test

fn main() {
    println!("Hello, world!");

    let number = 11;
    if 0<number && number < 5 {
        println!("condition was true");
    } else if 5<=number && number < 10 {
        println!("condition was false");
    }else{
        println!("condition was other result")
    }

    let_tes();
}

fn let_tes() ->i32 {
    let condition = true;
    let number = if condition {
        5
    }
    else{
        6
    };

    println!("The value of number is:{}", number);
    return number;
}
