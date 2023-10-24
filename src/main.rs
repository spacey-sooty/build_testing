extern "C" {
    fn add(a: u64, b: u64) -> u64;
}
fn main() {
    let sum = unsafe {
        add(1, 2)
    };

    println!("{}", sum);
}

