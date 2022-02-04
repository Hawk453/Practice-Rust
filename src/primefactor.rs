// Largest prime factor of an number
// Eg. the largest prime factor of 13195 is 29

fn prime_factors(n : u64) -> Vec<u64> {
    let mut v : Vec<u64> = Vec::new();

    let mut x : u64 = n;

    while x % 2 == 0 {
        v.push(2);
        x = x/2;
    }
     
    for index in (3.. x*x).step_by(2) {

        while x % index == 0 {

            v.push(index ); 
            x = x / index ;

        }

    }

    if x > 2 {
        v.push(x);
    }

    return v;
}


fn main(){
    let n:u64 = 13195 ;
    let  v : Vec<u64> =  prime_factors(n);
    let maxvalue = v.iter().max();
    match maxvalue {
        Some(max) => print!("Max prime factor is: {max}"),
        None => print!("Error, no max in vector"),
    }
}