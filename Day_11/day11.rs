use std::io::prelude::*;
use std::fs::File;
use std::cmp;
use std::iter::FromIterator;
use std::collections::HashMap;

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn blink_stone(input: String) -> (String, usize) {
    
    let mut vec = Vec::new();
    
    let items = input.split_whitespace();
    
    for i in items{
        let i_as_int = i.parse::<usize>().unwrap();
        
        if i == "0"{
            vec.push("1".to_string());
        } else if i.len() % 2 == 0{
            let mut cur = i.clone();
            let sub_len = i.len() / 2;
            while !cur.is_empty() {
                let (chunk, rest) = cur.split_at(cmp::min(sub_len, cur.len()));
                // Handle leading 0's automatically
                let temp_int = chunk.to_string().parse::<usize>().unwrap();
                vec.push(temp_int.to_string());
                cur = rest;
            }
        } else {
            let multipled_i = i_as_int * 2024;
            let multipled_i_str = multipled_i.to_string();
            vec.push(multipled_i_str);
        }
        
    }
    
    return (vec.join(" "), vec.len());
}

// I could just remove blink_stone(v1) - But *shrugs*
fn blink_stone_v2(stone_mapping: &mut HashMap<i64, usize>){
    
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.with_capacity
    let mut next_stone_mapping = HashMap::with_capacity(stone_mapping.capacity());

    for (&key, value) in stone_mapping.iter() {
        if key == 0 {
            *next_stone_mapping.entry(1).or_insert(0) += value;
        } else {
            let key_string = key.to_string();
            
            if key_string.len() % 2 == 0 {
                let mut cur = key_string.clone();
                let sub_len = key_string.len() / 2;
                while !cur.is_empty() {
                    let (chunk, rest) = cur.split_at(cmp::min(sub_len, cur.len()));
                    // Handle leading 0's automatically
                    let temp_int = chunk.to_string().parse::<i64>().unwrap();
                    *next_stone_mapping.entry(temp_int).or_insert(0) += value;
                    cur = rest.to_string();
                }

            } else {
                *next_stone_mapping.entry(key * 2024).or_insert(0) += value;
            }
        }
    }

    *stone_mapping = next_stone_mapping;
    //println!("{:?}", histogram)
}

#[allow(dead_code, unused_variables)]
fn part_one(){
    let string_value = read_example_txt();
    
    let mut test = string_value.clone();
    
    let mut number_of_stones = 0;
    
    let mut range = 25;
    
    while range >0{
    
        (test, number_of_stones) = blink_stone(test);
    
        //println!("{}", test);
        //println!("{}", number_of_stones);
        
        range -= 1;
    }
    
    println!("The answer to part one is {}", number_of_stones)
    
}


#[allow(dead_code, unused_variables)]
fn part_two(){
    let string_value = read_example_txt();
    
    let int_vec: Vec<i64> = string_value.split(' ').map(|x|->i64{x.parse().unwrap()}).collect();
    
    let mut stone_mapping = HashMap::from_iter(int_vec.into_iter().map(|s| (s,1)));

    let mut range = 75;
    
    while range >0{
    
        blink_stone_v2(&mut stone_mapping);
        
        range -= 1;
    }
    
    let number_of_stones: usize = stone_mapping.values().sum();
    
    println!("The answer to part two is {}", number_of_stones);
    
}

// https://www.onlinegdb.com/online_rust_compiler
fn main() {    
    part_one();

    part_two();
}