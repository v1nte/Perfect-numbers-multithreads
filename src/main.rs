fn is_primes(n) -> bool {
    if n==2{
        return true
    }

    if n <= 1 || n % 2 == 0 {
        return false
    }

    let md: u32 = pow(n,(1/2)) + 1;
    
    for i in (3..md).step_by(2) {
        if n % i == 0 {
            return false
        }
    }

    return true
}

fn pow(a, b) ->  i32 {
    for i in 1..b  {
        a = a * a;
    }
    return a
}

fn main() {

    
}
