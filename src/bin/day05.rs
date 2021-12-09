use std::{collections::HashMap, hash::Hash, cmp};

fn solve1(input: &str) -> i32 {
    let mut ocean_floor: HashMap<Point, i32> = HashMap::new();

    let vents = parse(input);

    for vent in vents {
        if vent.0.x == vent.1.x {
            let min = cmp::min(vent.0.y, vent.1.y);
            let max = cmp::max(vent.0.y, vent.1.y);
            for y in min..=max {
                let point = Point {
                    x: vent.0.x,
                    y: y,
                };
                if ocean_floor.contains_key(&point) {
                    ocean_floor.insert(point, ocean_floor.get(&point).unwrap()+1);
                }
                else {
                    ocean_floor.insert(point, 1);
                }
                
            }

        } else if vent.0.y == vent.1.y {
            let min = cmp::min(vent.0.x, vent.1.x);
            let max = cmp::max(vent.0.x, vent.1.x);
            for x in min..=max {
                let point = Point {
                    x: x,
                    y: vent.0.y,
                };
                if ocean_floor.contains_key(&point) {
                    ocean_floor.insert(point, ocean_floor.get(&point).unwrap()+1);
                }
                else {
                    ocean_floor.insert(point, 1);
                }
                
            }
        }
    }

    let mut count = 0;

    for entry in &ocean_floor {
        if *entry.1 >= 2 {
            count += 1;
        }
    }
    return count;
}
  
fn solve2(input: &str) -> i32 {
  
    let mut ocean_floor: HashMap<Point, i32> = HashMap::new();

    let vents = parse(input);

    for vent in vents {
        let (mut x1, mut y1) = (vent.0.x, vent.0.y);
        let (x2, y2) = (vent.1.x, vent.1.y);

        loop {
            let point = Point {
                x: x1,
                y: y1,
            };

            if ocean_floor.contains_key(&point) {
                ocean_floor.insert(point, ocean_floor.get(&point).unwrap()+1);
            } else {
                ocean_floor.insert(point, 1);
            }

            println!("{:?} -> {:?}", (x1,y1), (x2,y2));

            if (x1,y1) == (x2,y2) { break; }

            if x1 < x2 { x1 += 1 }
            if x1 > x2 { x1 -= 1 }
            if y1 < y2 { y1 += 1 }
            if y1 > y2 { y1 -= 1 }
        }
    }

    let mut count = 0;

    for entry in &ocean_floor {
        if *entry.1 >= 2 {
            count += 1;
        }
    }
    return count;
}

fn parse(input: &str) -> Vec<(Point, Point)> {

    let mut vents: Vec<(Point, Point)> = vec![];

    let lines = input.lines();

    for line in lines {
        let x1: i32 = line[0..line.find(",").unwrap()].parse().unwrap();
        let y1: i32 = line[line.find(",").unwrap()+1..line.find(" ").unwrap()].parse().unwrap();

        let line = line.split_at(line.find(">").unwrap()+2).1;

        let x2: i32 = line[0..line.find(",").unwrap()].parse().unwrap();
        let y2: i32 = line[line.find(",").unwrap()+1..].parse().unwrap();

        let point1 = Point{
            x: x1,
            y: y1,
        };

        let point2 = Point{
            x: x2,
            y: y2,
        };

        vents.push((point1, point2));

    }

    //println!("{:?}", vents);

    return vents;
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
  
  fn main() {
  
    let input = include_str!("day05.in");
  
    parse(input);

    println!("{:?}", solve1(input));
  
    println!("{:?}", solve2(input));
  }
  