use aoc2021::utils;

fn read_input(path: &str) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();
  let lines = utils::read_lines(path).unwrap();

  for line in lines {
    let l = line.unwrap();
    out.push(l);
  }

  out
}

fn part1(input: &Vec<String>) {
  unimplemented!();
}

fn part2(input: Vec<String>) {
  unimplemented!();
}

fn main() {
  let mut input = read_input("inputs/day3.txt");
  println!("Readsss {} input", input.len());

  part1(&input);
  part2(input);

}