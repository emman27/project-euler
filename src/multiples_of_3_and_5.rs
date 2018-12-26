fn is_multiple_of(n: u64) -> impl Fn(&u64) -> bool {
    move |m: &u64| *m % n == 0
}

pub fn is_multiple_of_5_or_3(n: &u64) -> bool {
    is_multiple_of(5)(n) || is_multiple_of(3)(n)
}

pub fn solve(max: u64) -> u64 {
    (1..max).into_iter().filter(is_multiple_of_5_or_3).sum()
}
