fn fizz_buzz(x: i32) {
    for i in 1..=x {
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

#[derive(Debug)]
// membuat enum untuk menampung data
enum FizzBuzz{
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i32),
}

// membuat function untuk menampilkan data dari enum
fn fizz_buzz_enum(i: i32) {
    match i {
        // mengecek apakah i sama dengan 3 dan 5
        // jika i sama dengan 3 dan 5 maka akan menampilkan FizzBuzz dimana mengambil value dari enum FizzBuzz
        i if i % 3 == 0 && i % 5 == 0 => println!("{:?}", FizzBuzz::FizzBuzz),
        // mengecek apakah i sama dengan 3
        // jika i sama dengan 3 maka akan menampilkan Fizz dimana mengambil value dari enum Fizz
        i if i % 3 == 0 => println!("{:?}", FizzBuzz::Fizz),
        // mengecek apakah i sama dengan 5
        // jika i sama dengan 5 maka akan menampilkan Buzz dimana mengambil value dari enum Buzz
        i if i % 5 == 0 => println!("{:?}", FizzBuzz::Buzz),
        // mengecek apakah i bukan sama dengan 3 dan 5
        // jika i bukan sama dengan
        // maka akan menampilkan Number dimana mengambil value dari enum Number
        _ => println!("{:?}", FizzBuzz::Number(i)),
    }
}

fn main() {
    fizz_buzz(100);
    palindrome_checker("kakak");
    for i in 1..=100 {
        fizz_buzz_enum(i);
    }
}
