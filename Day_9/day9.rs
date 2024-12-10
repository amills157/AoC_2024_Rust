use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn string_disk_space_representation(input: &str) -> (Vec<char>, usize) {

    let char_vec: Vec<char> = input.chars().collect();

    let mut id = 0;

    let mut free_space_count = 0;

    let mut vec = Vec::new();

    // This is ugly but using as to cast returns the ascii to int, rather than char to value as int
    for i in 0..char_vec.len() {
        if i % 2 == 0{

            // println!("index = {}", i);

            // println!("char = {}", char_vec[i]);

            let multipler = char_vec[i].to_string().parse::<usize>().unwrap();

            for j in 0..multipler{
                vec.push(char::from_digit(id as u32, 10).unwrap())
            }

            id += 1;

        } else {

            if char_vec[i] != '0'{
                let multipler = char_vec[i].to_string().parse::<usize>().unwrap();

                for j in 0..multipler{
                    vec.push('.');
                    free_space_count += 1;
                }

            }
        }

        //println!("{:?}", vec);
    }

    return (vec, free_space_count);

}


fn part_one(){
    let string_value = read_example_txt();

    let (mut disk_space_vec, mut free_space_count) = string_disk_space_representation(&string_value);
    
    println!("{:?}", disk_space_vec);

    let mut last_element_tracker = 1;

    // TODO: Probably easier to create a new array rather than try and be smart here
    for i in 0..disk_space_vec.len(){
        if disk_space_vec[i] == '.'{
            disk_space_vec[i] = disk_space_vec[disk_space_vec.len() - last_element_tracker];
            disk_space_vec.push('.');
            disk_space_vec.remove(disk_space_vec.len() - last_element_tracker);
            last_element_tracker += 1;
            free_space_count -= 1;

            println!("free space count = {:?}", free_space_count);

            if free_space_count == 0{
                break;
            }
                
        }
        println!("{:?}", disk_space_vec);
    }

    println!("{:?}", disk_space_vec);

    
    // let (mut int_vec_1, mut int_vec_2) = string_to_int_vectors(&string_value);

    // println!("{:?}", files);

    // println!("{:?}", free_space);

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