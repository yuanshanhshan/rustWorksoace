mod stack;
use stack::Stack;
// divide_by_two.rs
fn divide_by_two(mut dec_num: u32) -> String {
    // 用栈保存余数 rem
    let mut rem_stack = Stack::new();

    //rem 入栈
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    // pop stack to string
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }
    bin_str
}

// base_converter
fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();
    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}
fn main() {
    let bin_str: String = divide_by_two(10);
    println!("10 is b{bin_str}");
    let bin_str2: String = base_converter(100, 2);
    println!("100 is b{bin_str2}")
}
