fn solve1(input: &str) -> u128 {
  let mut fish_vec = parse(input);

  for x in 0..80 {
    fish_vec = simulate_day(fish_vec);      
  }

  let mut sum = 0;
  for x in fish_vec {
      sum += x;
  }

  return sum;
}

fn solve2(input: &str) -> u128 {
    let mut fish_vec = parse(input);

    for x in 0..256 {
      fish_vec = simulate_day(fish_vec);      
    }
  
    let mut sum = 0;
    for x in fish_vec {
        sum += x;
    }
  
    return sum;
}

fn simulate_day(fishes: [u128; 9]) -> [u128; 9] {
    let mut new_fishes = [0u128; 9];

    new_fishes[0] = fishes[1];
    new_fishes[1] = fishes[2];
    new_fishes[2] = fishes[3];
    new_fishes[3] = fishes[4];
    new_fishes[4] = fishes[5];
    new_fishes[5] = fishes[6];
    new_fishes[6] = fishes[0] + fishes[7];
    new_fishes[7] = fishes[8];
    new_fishes[8] = fishes[0];

    println!("{:?}", new_fishes);

    return new_fishes;

}

fn parse(input: &str) -> [u128; 9] {
    let mut fish_vec = [0u128; 9];

    for fish in input.split(",") {
        let day: usize = fish.parse().unwrap();
        fish_vec[day] = fish_vec[day] + 1;
    }

    println!("{}", fish_vec.len());
    

    return fish_vec;
}

fn main() {
  let input = include_str!("day06.in");

  println!("{:?}", solve1(input));

  println!("{:?}", solve2(input));
}
