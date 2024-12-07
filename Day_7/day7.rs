use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn string_to_calibrations(input: &str) -> BTreeMap<usize, Vec<usize>> {
    let mut calibrations = BTreeMap::<usize,Vec<usize>>::new();

    for line in input.lines() {

        let split = line.split_once(":").unwrap();
        // println!("{:?}", split.0.parse::<usize>().unwrap());
        
        // let nums: Vec<usize> = split.1.trim().split_whitespace().filter_map(|i| i.parse::<usize>().ok()).collect();

        // println!("{:?}", nums);

        calibrations
            .entry(split.0.parse().unwrap())
            .or_insert(split.1.trim().split_whitespace().filter_map(|i| i.parse::<usize>().ok()).collect());
    }

    return calibrations;
}


fn part_one(){
    let string_value = read_example_txt();

    let calibrations = string_to_calibrations(&string_value);

    let mut result = 0;

    'outer: for (key, value) in calibrations {

        //println!("{key}");

        //println!("{:?}", value);

        let number_of_permutations = 1usize << value.len() - 1;

        //println!("shift operator = {:?}", 1usize << n);

        'inner: for permutation in 1..=number_of_permutations {
            // println!("Permutation = {permutation}");

            let mut sum = value[0];

            for (idx, &i) in value.iter().skip(1).enumerate() {

                //bitwise AND
                // 0
                // 1
                // 10
                // 11
                // 100
                // 101
                // 110
                // 111
                // 1000
                // eg
                // 1 & 100 == 0 (1 & 4)
                // println!("idx == {idx}");

                // println!("shift == {:?}", 1 << idx);
                if permutation & (1 << idx) != 0 {
                    sum += i;
                    // println!("+");
                } else {
                    sum *= i;
                    // println!("*");
                }

                //println!("{sum}");

            }

            if sum == key {

                //println!("{sum} == {key}");

                result += sum;
                break 'inner;

            }
        }

        // println!("-----------------------");
        
    }

    println!("The answer to part one is {}", result);

    // int_vec_1.sort();
    // int_vec_2.sort();

    // let mut sum = 0;

    // for (pos, e) in int_vec_1.iter().enumerate() {
    //     // println!("Element at position {}: {:?}", pos, e);

    //     let diff = (e - int_vec_2[pos]).abs();

    //     sum += diff;

    //     //println!("{}", diff);

    // }

    // println!("The answer to part one is {}", sum);
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