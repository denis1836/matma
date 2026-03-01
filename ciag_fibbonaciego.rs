use std::io::{self, Write};

fn main() {
    let u_num = read_i32("Podaj ile chcesz wyrazów ciągu Fibonacciego: ");

    for i in 0..=u_num {
        if i < u_num {
            print!("{}, ", fib(i));
        } else {
            print!("{}", fib(i));
        }
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

fn read_i32(text: &str) -> i32 {
    loop {
        let mut u_i = String::new();

        print!("{}", text);
        io::stdout().flush().expect("failed to flush");
        io::stdin().read_line(&mut u_i).unwrap();

        match u_i.trim().parse::<i32>() {
            Ok(val) => return val,
            Err(err) => println!("ERROR: {}", err),
        };
    }
}