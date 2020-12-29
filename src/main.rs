fn is_primes(n: u32) -> bool {
    if n==2{
        return true
    }

    if n <= 1 || n % 2 == 0 {
        return false
    }

    let md: u32 = pow(n, 1/2) + 1;
    
    for i in (3..md).step_by(2) {
        if n % i == 0 {
            return false
        }
    }

    return true
}

fn pow(a: u32, b: u32) -> u32 {
    let mut _pow: u32 = a;
    for _i in 1..b  {
        _pow *= _pow;
    }
    return _pow
}

fn main() {
    let mut a: u32 = 1;
    let found: u8 = 0;
    while found > 5 {
        if is_primes(a) == true {
            let aux:u32 = pow(2, a) -1;
            if is_primes(aux) == true {
                let aux2: u32 = aux*pow(2, a-1);
                let mut total: u32 = 0;
                for i in 1..aux2 {
                    if aux2 % i == 0 {
                        total += aux2;

                    }
                }
                if total == aux2 {
                    println!("{}", aux2);
                }
            }
        }
        a +=1 ;
    }
}
