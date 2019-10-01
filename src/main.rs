use std::io;

fn main() {
    loop {
        println!("Please input Fibonacci N.");

        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n > 186 {
            println!("Sorry, please less then 186");
            continue;
        }


        println!("You Fibonacci N: {}", n);

        let mut left: u128 = 0;
        let mut right: u128 = 1;
        for _number in (1..n).rev() {
            let old = right;
            right += left;
            left = old;
        }
        println!(" F ({})= {}", n, right);
    }
}
