use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use itertools::Itertools;

// #[derive(Debug)] == Ability to print
// usize seems cause a panic - So switched to i32
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug)]
struct Antenna {
    coordinates: Coordinates,
    frequency: char,
}


fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn part_one(){
    let string_value = read_example_txt();

    let mut antennas = Vec::new();

    let mut antinodes: HashSet<Coordinates> = HashSet::new();

    let mut y_axis_max = 0;

    let mut x_axis_max = 0;

    for (y, line) in string_value.lines().enumerate() {
        y_axis_max += 1;

        if x_axis_max == 0 {
            x_axis_max = line.len()
        }

        //println!("{}", line);

        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                antennas.push(Antenna {
                    coordinates: Coordinates::new(x as i32, y as i32),
                    frequency: c,
                });
            }
        }
    }

    // println!("{:?}", antennas);

    // println!("line count = {line_count}");

    // println!("line len = {line_len}");

    // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.sorted_by
    let groups = antennas
        .iter()
        .sorted_by(|a, b| a.frequency.cmp(&b.frequency))
        .chunk_by(|a| a.frequency);

    for (frequency, group) in groups.into_iter() {
        //println!("{:?}",frequency);

        let locations = group.map(|a| &a.coordinates).collect_vec();
        //println!("{:?}",positions);
        // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.combinations
        let antenna_pairs = locations.iter().combinations(2).collect_vec();
        //println!("{:?}",antenna_pairs);
        for pair in antenna_pairs {
            let a = pair[0];
            let b = pair[1];

            let diff = Coordinates::new(a.x - b.x, a.y - b.y);

            //println!("Sub Postion = {:?}",diff);
            let check_nodes = vec![
                Coordinates::new(a.x + diff.x, a.y + diff.y),
                Coordinates::new(b.x - diff.x, b.y - diff.y),
                // Coordinates::new(a.x - diff.x, a.y - diff.y),
                // Coordinates::new(b.x + diff.x, b.y + diff.y)
                ];

            for node in check_nodes {
                // Needed for print / debug to work with adding to HashSet
                //let mut node_clone = node.clone();
                if (node.x >= 0 && node.x < x_axis_max as i32) && (node.y >= 0 && node.y < y_axis_max as i32) {
                    antinodes.insert(node);
                    //println!("Node = {:?}",node_clone);
                }
            }
        }
    }

    println!("The answer to part one is {}", antinodes.len());
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