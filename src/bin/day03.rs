fn solve1(input: &str) -> i32 {
  
  2
}

fn solve2(mut input: &str) -> Result<i64, &'static str> {

  input.sort_unstable();
}

fn main() {

  let mut input = include_str!("day03.in");

  let res = solve1(input);
  println!("{:?}", res);

  println!("{:?}", solve2(input));
}
