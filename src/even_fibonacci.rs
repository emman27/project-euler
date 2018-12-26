struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let tmp: u64 = self.a;
        self.a = self.b;
        self.b = tmp + self.a;
        Some(self.b)
    }
}

pub fn solve(max: u64) -> u64 {
    Fibonacci::new()
        .take_while(|val| *val <= max)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_to_89() {
        assert_eq!(44, super::solve(34))
    }
}
