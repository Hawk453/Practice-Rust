fn luhn(cc: &str) -> bool {
    let mut digit_count = 0;
    let mut sum = 0;

    for (i, ch) in cc.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(d) => {
                sum += if i%2 == 1 {
                    let dd = d * 2;
                    dd/10 + dd%10
                } else {
                    d
                };
                digit_count += 1;
            },

            None => return false,
        }
    }

    if digit_count < 2 {
        return false;
    }

    sum % 10 == 0
}

fn main() {
    let cc_number = "1234 5678 1234 5670";
    println!("The {cc_number} is valid: {}", if luhn(cc_number) {"yes"} else {"no "});
}