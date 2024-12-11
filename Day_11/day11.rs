use std::io::prelude::*;
use std::fs::File;
use std::cmp;


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

// https://www.onlinegdb.com/online_rust_compiler
fn main() {    
    part_one();

    //part_two();
}