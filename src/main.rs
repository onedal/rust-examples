fn main() {
    println!("Hello, world!");
    println!("123");
    let num = gcd(0,7);
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
    }
    n
}

