extern crate project_euler;

fn main() {
    println!(
        "Problem 1: {}",
        project_euler::multiples_of_3_and_5::solve(1000)
    );

    println!(
        "Problem 2: {}",
        project_euler::even_fibonacci::solve(4_000_000)
    );

    println!(
        "Problem 3: {}",
        project_euler::largest_prime_factor::largest_prime_factor(&mut 600851475143)
    );
}
