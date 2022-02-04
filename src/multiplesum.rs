// sum of multiple of 3 and 5 

fn main() {
    let mut v : Vec<u32> = Vec::new();

    let n : u32 = 10;

    for i in 1..n {
        if i % 5 == 0 || i % 3 == 0 {
            v.push(i);
        }
    }

    let sum: u32 = v.iter().sum();
    println!("{:?}", v);
    print!("The sum of the multiples is {sum}")
    
}