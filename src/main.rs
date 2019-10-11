use std::io::Write;
use std::str::FromStr;


fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
        .expect("error parsing argument!"));
        println!("num = {}", arg);
        
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER").unwrap();
        std::process::exit(1);
    }
    
    



    let num = gcd(81,9);
    println!("num = {}", num);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
        println!("{}",m);
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(81,9),9);
}
