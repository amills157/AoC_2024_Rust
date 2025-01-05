use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn John_Lewis(string_value: &String) -> (Vec<&str>, Vec<&str>){

    let mut split_value = "\r\n\r\n";
    
    let (temp_towels, temp_patterns) = string_value.trim().split_once(split_value).unwrap();
    
    let mut towels = Vec::new();
    
    for towel in temp_towels.split(", "){
        towels.push(towel);
    }
    
    let mut patterns = Vec::new();
    
    for line in temp_patterns.lines(){
        patterns.push(line);
    }
    
    // println!("{:?}", towels);
    
    // println!("{:?}", patterns);
    
    return (towels, patterns)
}

// <'a> == a liftime name - https://stackoverflow.com/questions/74031711/what-is-the-significance-of-the-name-a-in-all-of-rusts-reference-lifetime-an
fn check_design<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, usize>,) -> usize {
    let mut filtered_towels = Vec::new();
    let mut remaining_patterns = Vec::new();
    let mut valid_patterns = Vec::new();
    let mut sum = 0;

    if let Some(ans) = cache.get(pattern) {
        return *ans;
    }

    let n = pattern.len();

    if n == 0 {
        return 1;
    }
    
    // I could probably do this without endless for loops but no regex crate in the online compiler
    // Using the various .map , .iter() and .filter() will just leave me with code I'll weep over when I re-read
        
    for towel in towels{
        if pattern.starts_with(towel){
            filtered_towels.push(towel);
        }
    }

    
    for towel in filtered_towels {
        // https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices
        // If pattern is "brwrr" and towel is "br", this creates the slice "wrr"
        let patterns_to_check = &pattern[towel.len()..];
        remaining_patterns.push(patterns_to_check);
    }

    
    for pattern_to_check in remaining_patterns {
        if pattern_to_check.len() < n {
            valid_patterns.push(pattern_to_check);
        }
    }

    
    for pattern in valid_patterns {
        sum += check_design(pattern, towels, cache);
    }

    cache.insert(pattern, sum);

    return sum
}


fn part_one_and_two(){
    let string_value = read_example_txt();
    
    let (towels, patterns) = John_Lewis(&string_value);
    
    let mut cache = HashMap::default();
    
    let mut designs = Vec::new();
    
    for pattern in patterns{
        designs.push(check_design(pattern, &towels, &mut cache));
    }
    
    let answer: usize = designs.iter().filter(|n| **n > 0).count();
    println!("The answer to part one is {answer}");
    
    let sum: usize = designs.iter().sum();
    println!("The answer to part two is {sum}");
    
}


fn main() {    
    part_one_and_two();

    // Starting value (41644071) is too low. So I know at least I need to be working up.
    // I suspect that the use of modulo % 8 will be a key part in solving this smartly
    //part_two();

}