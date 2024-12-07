use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::cmp::Ordering;

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

fn part_one_and_two(){
    let string_value = read_example_txt();

    let printing = string_to_printing(&string_value);

    //println!("{:?}", rules.rules);

    // println!("{:?}", rules.updates);

    let mut pt1_sum = 0;

    let mut invalid_updates = vec![];

    // No goto in Rust
    // https://stackoverflow.com/questions/22905752/named-breaks-in-for-loops-in-rust
    'outer: for update in &printing.updates {
        //println!("{:?}", update);
        for idx in 0..update.len() {
            let current_page = &update[idx];
            if let Some(must_be_before) = printing.rules.get(current_page) {
                for pages in update.iter().skip(idx) {
                    if must_be_before.contains(pages) {
                        invalid_updates.push(update);
                        continue 'outer;
                    }
                }
            }
        }
        pt1_sum += update[update.len() / 2];

        // println!("--------------");
    }
    println!("The answer to part one is {}", pt1_sum);

    let mut pt2_sum = 0;

    for update in invalid_updates {
        // Need to implictly clone / copy data 
        // update` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        let mut update_clone = update.clone();
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
        update_clone.sort_by(|a, b| {
            if let Some(must_be_before) = printing.rules.get(b) {
                if must_be_before.contains(a) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        pt2_sum += update_clone[update_clone.len() / 2]
    }
    println!("The answer to part two is {}", pt2_sum);

}


fn main() {    
    part_one_and_two();
}