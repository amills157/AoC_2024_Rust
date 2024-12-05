use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

// Struct time ya'll
struct Printing {
    rules: BTreeMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn string_to_printing(input: &str) -> Printing {
    let mut rules = BTreeMap::new();
    let mut updates = vec![];

    for line in input.lines() {
        if line.contains("|") {
            let rule = line.split_once("|").unwrap();
            rules
                .entry(rule.1.parse().unwrap())
                .or_insert_with(Vec::new)
                .push(rule.0.parse().unwrap());
            //println!("{:?}", rules);
        }
        if line.contains(",") {
            updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    Printing { rules, updates }
}

fn part_one(){
    let string_value = read_example_txt();

    let printing = string_to_printing(&string_value);

    //println!("{:?}", rules.rules);

    // println!("{:?}", rules.updates);

    let mut sum = 0;

    // No goto in Rust
    // https://stackoverflow.com/questions/22905752/named-breaks-in-for-loops-in-rust
    'outer: for update in &printing.updates {
        //println!("{:?}", update);
        for idx in 0..update.len() {
            let current_page = &update[idx];
            if let Some(must_be_before) = printing.rules.get(current_page) {
                for pages in update.iter().skip(idx) {
                    if must_be_before.contains(pages) {
                        continue 'outer;
                    }
                }
            }
        }
        sum += update[update.len() / 2];

        // println!("--------------");
    }
    println!("The answer to part one is {}", sum);

}


// fn part_two(){
//     let string_value = read_example_txt();

//     let (mut int_vec_1, mut int_vec_2) = string_to_int_vectors(&string_value);

//     int_vec_1.sort();
//     int_vec_2.sort();

//     let mut sum = 0;

//     for (pos, e) in int_vec_1.iter().enumerate() {
//         // println!("Element at position {}: {:?}", pos, e);

//         let value_count = (int_vec_2.iter().filter(|&n| *n == *e).count()) as i32;

//         sum += (value_count * e);

//         //println!("{}", (value_count * e));
//     }

//     println!("The answer to part two is {}", sum);

// }


fn main() {    
    part_one();

    //part_two();
}