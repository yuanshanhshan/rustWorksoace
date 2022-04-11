struct Fib(usize, usize);

impl Fib {
    fn new() -> Fib {
        Fib(0, 1)
    }
}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        *self = Fib(self.1, self.0 + self.1);
        Some(self.0)
    }
}
fn main() {
    let last = 20;
    println!(
        "fib({}) result: {:?}",
        last,
        Fib::new().take(last).collect::<Vec<usize>>()
    )
}
