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
    // membuat string kosong
    let mut value_reversed = String::new();

    // membuat for loop untuk mengambil karakter dari parameter untuk dimasukkan pada string kosong yang sudah dibuat
    // yang kemudian direverse
    for c in _x.chars().rev() {
        value_reversed.push(c);
    }

    // mengecek apakah string yang dibuat sama dengan string dari parameter
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
