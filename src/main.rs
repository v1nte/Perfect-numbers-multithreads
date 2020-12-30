
fn main() {
    let total: u32 = 61;
    let mut value: u64;
    println!("p\t Number\n");

    for i in 1..total+1 {
        value = is_perfect(i);
        if value != 0 {
            println!("{}\t {}", i, value);
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n==2{
        return true
    }

    if n <= 1 || n % 2 == 0 {
        return false
    }

    let m = (n as f64).sqrt();
    let maxdiv = m + 1.0_f64;

    for i in (3..(maxdiv as u64)).step_by(2) {
        if n % i == 0 {
            return false
        }
    }

    return true
}

fn is_perfect(a: u32) -> u64 {
    if is_prime(a as u64) == true {
        let aux: u64 = 2_u64.pow(a) - 1;

        if is_prime(aux) == true {
            let val: u64 = aux * 2_u64.pow(a - 1);
            return val
        }
    }
    0
}
