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

fn part1(nums: &Vec<String>) {
  let mut horizontal_position = 0;
  let mut depth: i32 = 0;
  for num in nums {
    let s: Vec<&str> = num.split_whitespace().collect();
    let direction = s[0];
    let steps = s[1].parse::<i32>().unwrap();
    match direction {
      "forward" => horizontal_position += steps,
      "up" => depth -= steps,
      "down" => depth += steps,
      _ => println!("issue")
    }
  }
  println!("Part 1 Result is {}", horizontal_position*depth);
}


fn part2(nums: &Vec<String>) {
  let mut horizontal_position = 0;
  let mut depth: i32 = 0;
  let mut aim: i32 = 0;
  for num in nums {
    let s: Vec<&str> = num.split_whitespace().collect();
    let direction = s[0];
    let steps = s[1].parse::<i32>().unwrap();
    match direction {
      "forward" => {
        horizontal_position += steps;
        depth += aim*steps;
      }
      "up" => aim -= steps,
      "down" => aim += steps,
      _ => println!("issue")
    }
  }
  println!("Part 2 Result is {}", horizontal_position*depth);
}

fn main() {
  let  nums = read_input("inputs/day2-small.txt");
  println!("Readsss {} nums", nums.len());

  part1(&nums);
  part2(&nums);

}