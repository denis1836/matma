use std::io::{self, Write};

fn cz_1(n: &i32) {
    let mut x = *n;
    let mut i = 2;

    while x > 1 {
        while x % i == 0 {
            print!("{} ", i);
            x /= i;
        }
        i += 1;
    }

    println!();
}


fn nwd(p_a: &i32, p_b: &i32) -> i32 {
    let mut a: i32 = *p_a;
    let mut b: i32 = *p_b;
    
    while b != 0
    {
        let tmp: i32 = b;
        b = a % b;
        a = tmp;
    }
    
    a
}

fn main() {
    let n: i32 = read_i32();
    let m: i32 = read_i32();

    println!("Rozkład {}: ", n);
    cz_1(&n);
    
    println!("Rozkład {}: ", m);
    cz_1(&m);

    println!("NWW liczb {} i {} wynosi: {}", n, m, ((n * m) / nwd(&n, &m)) );
}

fn read_i32 () -> i32 { loop {
    let mut u_i = String::new();
    
    print!("Podaj liczbę: ");
    io::stdout().flush().expect("failed to flush");
    io::stdin().read_line(&mut u_i).unwrap();
    
    let n = match u_i.trim().parse::<i32>(){
        Ok(val) => {
            return val;
        }
        Err(_err) => {
            println!("ERROR: {}", _err);
        }
    };
    
    n
}}