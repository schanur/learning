fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for number in 1..4 {
        println!("{}!", number);
    }
    println!("NO LIFTOFF!!!");

    for n in 1..100 {
        println!("n: {}; fib(n): {}", n, fib(n));
    }

}

fn fib(n: u64) -> u64
{
    let mut last_result = 1;
    let mut result = 1;

    for multiplier in 1..n {
        let new_result = last_result + result;
        last_result = result;
        result = new_result;
    }

    result
}

