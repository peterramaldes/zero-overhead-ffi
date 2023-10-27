extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { abs(-192087) });
}
