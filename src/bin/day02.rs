fn solve1(input: &str) -> i32 {

    let input = input.lines();

    let mut directions: Vec<(&str,i32)> = Vec::new();

    for line in input {
      let tuple = line.split_at(line.find(" ").unwrap()+1);
      directions.push((tuple.0, tuple.1.parse().unwrap()));
    }

    println!("{:?}", directions);

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    
    for direction in directions {
     match direction.0 {
       "forward " => { 
         horizontal += direction.1;
         depth += aim*direction.1;
       }
       "up " => aim -= direction.1,
       "down " => aim += direction.1,
       _ => println!("error?")
     }
    }

    return depth*horizontal;
}

fn solve2(input: &str) -> i32 {
/*   input
    .lines()
    .map(|f| f.parse().ok())
    .collect::<Option<Vec<_>>>()
    .and_then(|nums| {
      Some(
        nums.windows(3)
          .map(|n| n.iter().sum())
          .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > prev { sum +1 } else { sum }, n)
          })
          .0
      )
    })
    .unwrap() */

    return 2;
}

fn main() {

  let input = include_str!("day02.in");

  println!("{:?}", solve1(input));

  println!("{:?}", solve2(input));
}
