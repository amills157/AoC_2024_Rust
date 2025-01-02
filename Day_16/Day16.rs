use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::AddAssign;



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    
}


impl Add for Point{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Racer {
    pos: Point,
    dir: (i32, i32),
}


impl Racer {
    fn new(start: Point) -> Self {
        Self {
            pos: start,
            dir: (1, 0),
        }
    }

    fn move_forward(&self) -> Self {
        Self {
            pos: self.pos + Point::new(self.dir.0, self.dir.1),
            dir: self.dir,
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            pos: self.pos,
            dir: (self.dir.1, -self.dir.0),
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            pos: self.pos,
            dir: (-self.dir.1, self.dir.0),
        }
    }
}


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn race_map(string_value: String) -> HashMap<Point, char>{

    let mut map: HashMap<Point, char> = HashMap::new();
    
    for (y, line) in string_value.lines().enumerate() {

        for (x, c) in line.chars().enumerate() {
            map.entry(Point::new(x as i32, y as i32)).or_insert(c);
        }
        
    } 
        
    return map
}

// https://doc.rust-lang.org/std/option - Allows return in case value isn't found
fn navigator(map: &HashMap<Point, char>, marker: char) -> Option<Point> {
    
    for (point, char) in map.into_iter(){
        if *char == marker{
            return Some(*point);
        }
    }
    
    None
    
}


fn race_blazing(map: &HashMap<Point, char>, start: Point, end: Point,) -> (i32, Vec<Vec<Point>>) {

    let mut best_score = 9999999;
    let mut best_paths = Vec::new();
    let mut visited = HashMap::new();
    let mut queue = VecDeque::with_capacity(10000);
    
    queue.push_front((0, Racer::new(start), vec![start]));
    
    // pop_front was getting caught in a loop. Used pop_back. Could also have changed the push order
    // This seems counterintuitive to how I want to handle moves. But works.
    while let Some((score, racer, mut path)) = queue.pop_back() {
        // We've gone above the current best score, so abandon the path (dis is not the way)
        if score > best_score {
            continue;
        }

        if let Some(&prev_score) = visited.get(&racer) {
            // We've already been here and with a lower score (dis is not the way)
            if prev_score < score {
                continue;
            }
        }
        
        // Otherwise dis might be the way
        visited.insert(racer.clone(), score);

        if racer.pos == end {
            if score == best_score {
                best_paths.push(path);
            } else {
                best_score = score;
                // https://doc.rust-lang.org/std/macro.vec.html (vec![] ~ dynamic Vec::new)
                best_paths = vec![path];
            }
            continue;
        }
        
        // Assume a left or right turn
        queue.push_front((score + 1000, racer.turn_left(), path.clone()));
        queue.push_front((score + 1000, racer.turn_right(), path.clone()));

        let next_tile = racer.move_forward();

        if map.get(&next_tile.pos) != Some(&'#') {
            path.push(next_tile.pos);
            // Add a move forward infront of the turn if we can
            queue.push_front((score + 1, next_tile, path));
        }
    }

    return (best_score, best_paths)

}


fn part_one_and_two(){
    let string_value = read_example_txt();
    
    let mut map = race_map(string_value);
    
    // for row in map{
    //     println!("{:?}", row);
    // }
    
    let start = navigator(&map, 'S').unwrap();
    let end = navigator(&map, 'E').unwrap();
    
    //println!("{:?}", start);
    
    //println!("{:?}", end);
    
    let (best_score, best_paths) = race_blazing(&map, start, end);
    
    
    println!("The answer to part one is {:?}", best_score);
    
    let mut unique_tiles = Vec::new();
    
    for mut paths in best_paths{
        for tile in paths{
            unique_tiles.push(tile);
        }
    }
    
    unique_tiles.sort();
    unique_tiles.dedup();
    
    println!("The answer to part two is {:?}", unique_tiles.len());
    
}


fn main() {    
    part_one_and_two();

}