use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::BTreeMap;

fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn calc_matches(input: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut num = 1;
    for num_str in re.find_iter(input){
        num *= num_str.as_str().parse::<i32>().unwrap();
    }
    num
}

fn filter_mults(multiplications: &BTreeMap<usize,String>, do_instructions: &[(usize,usize)]) -> BTreeMap<usize,String> {
    let mut filtered_multiplications = BTreeMap::<usize,String>::new();
    let mut do_iter = do_instructions.iter();
    let mut current_pair = do_iter.next().unwrap();
    let (mut start, mut end) = *current_pair;

    for (key, val) in multiplications {
        while *key > end {
            current_pair = match do_iter.next() {
                Some(pair) => pair,
                None => return filtered_multiplications,
            };
            start = current_pair.0;
            end = current_pair.1;
        }
        if *key >= start && *key <= end {
            filtered_multiplications.insert(*key, val.to_string());
        }
    }
    filtered_multiplications
}

fn part_one(){
    let string_value = read_example_txt();

    let mut sum = 0;
    let re = Regex::new(r"mul[(][0-9]{1,3},[0-9]{1,3}[)]").unwrap();

    for mat in re.find_iter(&string_value){
        
        let match_string = mat.as_str().to_string();
        
        sum += calc_matches(&match_string);
    }
    println!("The answer to part one is {}", sum);
}


fn part_two(){
    let string_value = read_example_txt();

    let mut sum = 0;
    let mul_regex = Regex::new(r"mul[(][0-9]{1,3},[0-9]{1,3}[)]").unwrap();

    let mut multiplications = BTreeMap::<usize,String>::new();

    for mat in mul_regex.find_iter(&string_value){
        multiplications.insert(
            mat.start(),
            mat.as_str().to_string()
          );
    }

    let do_regex = Regex::new(r"do[(][)]").unwrap();
    let dont_regex = Regex::new(r"don't[(][)]").unwrap();
    
    let mut do_captures = do_regex.find_iter(&string_value).map(|x| x.start());
    let mut dont_captures = dont_regex.find_iter(&string_value).map(|x| x.start());
    
    let mut do_instructions = Vec::<(usize, usize)>::new();

    let mut current_do = Some(0);
    let mut current_dont = dont_captures.next();
    
    let mut store_next = true;

    let mut last_do_pushed = false;

    while current_do.is_some() && current_dont.is_some() {
        if current_dont.unwrap() <  current_do.unwrap() {
            store_next = true;
            current_dont = dont_captures.next();
        } else {
            if store_next {
                do_instructions.push((current_do.unwrap(), current_dont.unwrap()));
                store_next = false;
                last_do_pushed = true;
            }
            current_do = do_captures.next();
            last_do_pushed = false;
        }
    }

    // Hacky fix to get the last do that was needed in the example but wasn't for the input and broke it
    // if !last_do_pushed{
    //     do_instructions.push((current_do.unwrap(), 99999999999999));
    // }
    
    
    //println!("{:?}", do_instructions);

    let results = filter_mults(&multiplications, &do_instructions);

    //println!("{:?}", results);

    for (pos, e) in results.iter().enumerate() {
        //println!("Element at position {}: {:?}", pos, e.1);

        sum += calc_matches(&e.1);
    }

    println!("The answer to part two is {}", sum);

}


fn main() {    
    part_one();

    part_two();
}