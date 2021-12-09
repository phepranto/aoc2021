/* fn solve_part1(&str input:<str>) {
  let iter = input.lines();

  let depth = iter.next();

  let depth = Some(depth);
} */

fn solve1(input: &str) -> i32 {
  input
    .lines()
    .map(|f| f.parse().unwrap())
    .fold((0, i32::MAX), |(sum, prev), n| {
      (if n > prev { sum + 1 } else { sum }, n)
    })
    .0
}

fn solve2(input: &str) -> i32 {
  input
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
    .unwrap()
}

fn main() {

  let input = include_str!("day01.in");

  let res = solve1(input);
  println!("{:?}", res);

  println!("{:?}", solve2(input));
}
