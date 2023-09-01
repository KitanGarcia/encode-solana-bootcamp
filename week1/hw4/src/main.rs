fn fizzbuzz() -> u8 {
    let mut fizzbuzz_count = 0;
    for n in 1..=301 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizz buzz");
            fizzbuzz_count += 1;
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        }
    }
    return fizzbuzz_count;
}

fn main() {
    println!("Welcome to fizzbuzz!");
    let fizzbuzz_count = fizzbuzz();
    println!("Fizz buzz count: {fizzbuzz_count}");
}
