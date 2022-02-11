use std::time::{SystemTime};
use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("../../../js/two_sum/numbers.txt").expect("Something went wrong reading the file");

    let mut numbers: Vec<i32> = Vec::new();
    let split: Vec<&str> = contents.split(",").collect();
    
    for i in split {
        numbers.push(i.parse::<i32>().unwrap());
    }

    two_sum(&numbers, 13);
}

fn _two_sum_naive(numbers: &Vec<i32>, target: i32) {
    let start = SystemTime::now();
    for number in numbers {
        // println!("{:#?}", number);
        for number2 in number+1..numbers.len() as i32 {
            // println!("{:#?}", number2);
            if number + number2 == target {
                let end = SystemTime::now();
                println!("Rust Solution: {:#?}, {:#?}", number, number2);
                println!("Rust Time: {:#?}ms", end.duration_since(start).unwrap().as_millis());
                return ();
            }
        }
    }
}

fn two_sum(numbers: &Vec<i32>, target: i32) {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    let mut sums: Vec<Vec<i32>> = Vec::new();

    let start = SystemTime::now();
    for (_index, number) in numbers.iter().enumerate() {
        let complement = target - number;
        if hash_map.contains_key(&complement) {
            sums.push(vec![number.to_owned(), complement]);
            println!("Possible: {:#?}, {:#?}", complement, number);
        } else {
            hash_map.insert(number.to_owned(), complement);
        }
    }
    let end = SystemTime::now();
    println!("Rust Time: {:#?}ms", end.duration_since(start).unwrap().as_millis());

    return ();
}
