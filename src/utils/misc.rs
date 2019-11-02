///! # Miscellaneous utilities
///!
///! Utility functions that do not belong in any single large categories

/// Run a fizzbuzz program, starting from 1 up to the provided `limit`. The program prints "Fizz"
/// upon hitting a number divisible by 3, "Buzz" if divisible by 5, and "FizzBuzz" if divisible by
/// both 3 and 5.
///
/// # Examples
///
/// ```Rust
/// fizzbuzz(100);
///
/// // Fizz
/// // Buzz
/// // ...
/// ```
fn fizzbuzz(limit: u32) {
    for _i in 1..limit {
        if _i % 15 == 0 {
            println!("Divisible by both 5 and 3: FizzBuzz!");
        } else if _i % 5 == 0 {
            println!("Divisible by 5: Buzz");
        } else if _i % 3 == 0 {
            println!("Divisible by 3: Fizz");
        }
    }
}

/// Run a fibonacci program, printing Fibonacci numbers `count` amount of times starting from 0.
///
/// # Examples
///
/// ```Rust
/// fibonacci(10);
///
/// // 0
/// // 1
/// // 1
/// // 2
/// // ...
/// ```
fn fibonacci(count: u32) {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;

    println!("{}", n1);
    println!("{}", n2);

    for _i in 1..count {
        let n3 = n1 + n2;
        n1 = n2;
        n2 = n3;

        println!("{}", n3);
    }
}

/// Convert between Fahrenheit and Celsius temperature.
///
/// # Examples
///
/// ```Rust
/// println!("{}", convert_temperature_fc(100., 'F'));
///
/// // 37.77777777777778
/// ```
fn convert_temperature_fc(temperature: f64, from: char) -> f64 {
    if from == 'F' {
        return (temperature - 32.) * (5. / 9.);
    }

    temperature * (9. / 5.) + 32.
}

/// Grab the nth word of a string.
///
/// # Examples
///
/// ```Rust
/// println!("The #{} word of '{}' is '{}'", 3, "This is a triumph", grab_nth_word("This is a triumph", 3));
///
/// // The #3 word of 'This is a triumph' is 'a'
/// ```
fn grab_nth_word(s: &str, n: u32) -> &str {
    let s_bytes = s.trim().as_bytes();

    // Handle empty string
    if s_bytes.len() == 0 {
        return "";
    }

    let mut prev_word_start_index: usize = 0;
    let mut word_count: u32 = 1;

    for (_i, &letter) in s_bytes.iter().enumerate() {
        if letter == b' ' {
            if word_count == n {
                return &s[prev_word_start_index.._i];
            }

            prev_word_start_index = _i + 1;
            word_count += 1;
        }
    }

    &s[..]
}
