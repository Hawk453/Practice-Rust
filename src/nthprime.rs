

//Nth prime number using optmized version of seive of eratosthenes

fn get_nth_prime(nth: u32) -> u32{

    let mut total_prime: u32 = 0;
    let mut size_factor: u32 = 2;

    let mut s : u32 = nth * size_factor;
    let mut primes : Vec<u32> = Vec::new();
    
    
    let  n: u32 = nth ;
    //let mut x : u32 = 0;
    while total_prime < n {
        primes = get_primes(s).iter().copied().collect();
        
        total_prime = primes[2..].iter().sum();
        size_factor +=1;
        s = n * size_factor;
    }

    let nth_prime = count_prime(primes, n).unwrap();


    return nth_prime;
}

fn get_primes(s : u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![1; s as usize];

    for index in 2..s {
        if v[index as usize] == 1 {
            for j in index..s {
                if index * j < s {
                    v[(index*j) as usize ] = 0;
                }
                else {
                    break;
                }
            }
        }
    }
    return v;
}


fn count_prime(primes : Vec<u32>, n : u32) ->Option<u32> {
    let mut counter: u32 = 0;
    for i in 2..primes.iter().count() {
        counter = counter + primes.iter().nth(i).unwrap();
        if counter == n {
            return Some(i as u32);
        }
    }
    return None;
}


fn main() {

    let nth: u32 = 6;
    let nth_prime = get_nth_prime(nth);
    print!("The nth prime is: {nth_prime}");

}