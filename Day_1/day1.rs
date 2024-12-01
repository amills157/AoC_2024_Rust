use std::io::prelude::*;
use std::fs::File;

fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn string_to_int_vectors(input: &str) -> (Vec<i32>, Vec<i32>) {

    let mut int_vec_1 = vec![];
    let mut int_vec_2 = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();

        int_vec_1.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
        int_vec_2.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
     
    }

    return (int_vec_1, int_vec_2);

}

fn part_one(){
    let string_value = read_example_txt();

    let (mut int_vec_1, mut int_vec_2) = string_to_int_vectors(&string_value);

    int_vec_1.sort();
    int_vec_2.sort();

    let mut sum = 0;

    for (pos, e) in int_vec_1.iter().enumerate() {
        // println!("Element at position {}: {:?}", pos, e);

        let diff = (e - int_vec_2[pos]).abs();

        sum += diff;

        //println!("{}", diff);

    }

    println!("The answer to part one is {}", sum);
}


fn part_two(){
    let string_value = read_example_txt();

    let (mut int_vec_1, mut int_vec_2) = string_to_int_vectors(&string_value);

    int_vec_1.sort();
    int_vec_2.sort();

    let mut sum = 0;

    for (pos, e) in int_vec_1.iter().enumerate() {
        // println!("Element at position {}: {:?}", pos, e);

        let value_count = (int_vec_2.iter().filter(|&n| *n == *e).count()) as i32;

        sum += (value_count * e);

        //println!("{}", (value_count * e));
    }

    println!("The answer to part two is {}", sum);

}


fn main() {    
    part_one();

    part_two();
}