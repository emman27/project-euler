fn is_prime(n: u64) -> bool {
    for x in 2..(n as f64).sqrt() as u64 + 1 {
        if n % x == 0 {
            return false;
        }
    }
    true
}

pub fn largest_prime_factor(n: &mut u64) -> u64 {
    let mut largest = 1;
    for x in 2..(*n as f64).sqrt() as u64 + 1 {
        if is_prime(x) {
            while *n % x == 0 {
                *n = *n / x;
                largest = x;
            }
            if *n == 1 {
                return largest;
            }
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    #[test]
    fn largest_of_13195() {
        assert_eq!(29, super::largest_prime_factor(&mut 13195))
    }
}
