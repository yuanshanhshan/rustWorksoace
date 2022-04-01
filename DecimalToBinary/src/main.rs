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
fn main() {
    let bin_str: String = divide_by_two(10);
    println!("10 is b{bin_str}");
}
