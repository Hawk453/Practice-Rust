use std::collections::HashSet;
use std::io;

fn check_collisions(mapping: &[usize], words: &Vec<String>) -> String {
    let mut encoded_set: HashSet<String> = HashSet::new();
    for word in words {
        let encoded_word: String = word.chars().map(|c| mapping[c as usize - 65].to_string()).collect();
        if encoded_set.contains(&encoded_word) {
            return "YES".to_string();
        } else {
            encoded_set.insert(encoded_word);
        }
    }
    "NO".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Failed to parse test cases");

    for case in 1..=t {
        let mut digits = String::new();
        io::stdin().read_line(&mut digits).expect("Failed to read digits");
        let mapping: Vec<usize> = digits.split_whitespace()
            .map(|x| x.parse().expect("Failed to parse digit"))
            .collect();

        let mut words = Vec::new();
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read N");
        let n: usize = n.trim().parse().expect("Failed to parse N");
        for _ in 0..n {
            let mut word = String::new();
            io::stdin().read_line(&mut word).expect("Failed to read word");
            words.push(word.trim().to_string());
        }

        let result = check_collisions(&mapping, &words);
        println!("Case #{}: {}", case, result);
    }
}
