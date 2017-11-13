use std::env;
use std::cmp;

fn ex_gcd(r0:i64, r1:i64, a0:i64, a1:i64, b0:i64, b1:i64) -> (i64, i64, i64) {
    let r2 = r0 % r1;

    if r2 == 0 {
        return (r1, a1, b1);
    }

    let q = r0 / r1;
    let a2 = a0 - q * a1;
    let b2 = b0 - q * b1;
    
    return ex_gcd(r1, r2, a1, a2, b1, b2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("2 arguments are needed");
    } else if args.len() > 3 {
        println!("too many arguments");
    }
    let n1 : i64 = args[1].parse().ok().expect("Invalid Number in arguments");
    let n2 : i64 = args[2].parse().ok().expect("Invalid Number in arguments");
    println!("n1 = {}, n2 = {}", n1, n2);
    if n1 ==0 || n2 == 0 {
        println!("arguments should be bigger than 0");
    }
    let r0 = cmp::max(n1, n2);
    let r1 = cmp::min(n1, n2);
    let (r, a, b) = ex_gcd(r0, r1, 1, 0, 0, 1);

    println!("r = {}, a = {}, b = {}", r, a, b);
    println!("{} * {} + {} * {} = {}", a, r0, b, r1, a * r0 + b * r1);
}
