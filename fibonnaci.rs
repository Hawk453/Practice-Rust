//even fibonnacu numbers

fn fibonacci(n : i32) -> i32 {
    let mut result: i32 = 2;
    let mut fib3: i32 = 2;
    let mut fib6: i32 = 0;
    let mut sum : i32 = 0;
    
    while result < n as i32 {
        sum = sum + result;
        result = 4*fib3 + fib6;
        fib6 = fib3;
        fib3 = result;
        
    }

    return sum;
}

fn main() {
    let  n : i32 = 100;
    println!("{}", fibonacci(n));
}