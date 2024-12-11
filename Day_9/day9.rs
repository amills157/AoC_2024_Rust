use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::iter::FromIterator;


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

// This is ugly - But it works. Multiple returns make it easier to keep track of files v free space
fn string_disk_space_representation(input: &str) -> (Vec<String>, Vec<usize>, usize) {

    let char_vec: Vec<char> = input.chars().collect();

    let mut id = 0;

    let mut free_space_count = 0;

    let mut vec = Vec::new();
    
    let mut usize_vec = Vec::new();

    for i in 0..char_vec.len() {
        if i % 2 == 0{

            let multipler = char_vec[i].to_string().parse::<usize>().unwrap();
            
            if multipler != 0{
                let id_str: String = id.to_string();
                for j in 0..multipler{
                    vec.push(id_str.clone());
                    usize_vec.push(id);
                }
            }

            id += 1;

        } else {

            if char_vec[i] != '0'{
                let multipler = char_vec[i].to_string().parse::<usize>().unwrap();

                for j in 0..multipler{
                    vec.push(".".to_string());
                    free_space_count += 1;
                }

            }
        }

    }

    return (vec, usize_vec, free_space_count);

}

#[allow(dead_code, unused_variables)]
fn part_one(){
    let string_value = read_example_txt();

    let (mut disk_space_vec, mut files_only, mut free_space_count) = string_disk_space_representation(&string_value);
    
    //println!("{:?}", disk_space_vec);

    let mut last_element_tracker = 1;
    
    let mut sorted_disk_space_vec = Vec::new();

    for i in 0..disk_space_vec.len(){
        if disk_space_vec[i] == "."{
            let usize_to_push = files_only[files_only.len() - last_element_tracker];
            sorted_disk_space_vec.push(usize_to_push);
            last_element_tracker += 1;
                
        } else {
            sorted_disk_space_vec.push(disk_space_vec[i].to_string().parse::<usize>().unwrap());
        }
        
        //println!("{:?}", sorted_disk_space_vec);
        
        if (sorted_disk_space_vec.len() + free_space_count) == disk_space_vec.len(){
            break;
        }
    }
    
    let mut sum = 0;
    
    for i in 0..sorted_disk_space_vec.len(){
        sum += i * sorted_disk_space_vec[i]
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

// https://www.onlinegdb.com/online_rust_compiler
fn main() {    
    part_one();

    //part_two();
}