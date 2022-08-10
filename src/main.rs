fn fizz_buzz(x: i32) {
    for i in 1..x {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn palindrome_checker(_x: &str) {
    let mut value_reversed = String::new();
    for c in _x.chars().rev() {
        value_reversed.push(c);
    }
    if _x == value_reversed {
        println!("{} ini palindrome", _x);
    } else {
        println!("{} ini bukan palindrome", _x);
    }
}
fn main() {
    // fizz_buzz(101);
    palindrome_checker("kakak");
}
