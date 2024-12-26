use std::io::prelude::*;
use std::fs::File;

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn sequence(secret: usize) -> usize{

    let value = secret * 64;
    let secret = prune(mix(secret, value));

    let value = secret / 32;
    let secret = prune(mix(secret, value));

    let value = secret * 2048;
    let secret = prune(mix(secret, value));
    
    return secret;
}


fn mix(secret: usize, value: usize) -> usize {
    return secret ^ value;
}

fn prune(secret: usize) -> usize {
    return secret.rem_euclid(16777216);
}

fn part_one(){
    let string_value = read_example_txt();
    
    let mut sum = 0;
    
    for value in string_value.lines(){
        let mut temp = value.parse::<usize>().unwrap();
        for _ in 0..2000{ 
            temp = sequence(temp);
        }
        println!("{}: {}", value, temp);
        
        sum += temp;
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

// https://www.onlinegdb.com/online_rust_compiler#
fn main() {    
    part_one();

    //part_two();
}