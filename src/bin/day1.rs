use aoc2021::utils;

fn read_ints(path: &str) -> Vec<i32> {
  let mut out: Vec<i32> = Vec::new();
  let lines = utils::read_lines(path).unwrap();

  for line in lines {
    let num = line.unwrap().parse::<i32>().unwrap();
    out.push(num);
  }

  out
}

fn part1(nums: &Vec<i32>) {
  let mut counter = 0;
  for n in 1..nums.len() {
    if nums[n] > nums[n-1] {
        counter+=1;
    }
  }
  println!("Part 1) There are {} increased entry", counter)
}


fn part2(nums: &Vec<i32>) {
    let mut counter = 0;
    let mut current_val = nums[0] + nums[1] + nums[2];
    for n in 3..nums.len() {
      let next_val = current_val + nums[n] - nums[n-3];
      if next_val > current_val {
          counter+=1;
      }
      current_val = next_val;
    }
    println!("Part 2) There are {} increased entry", counter)
}

fn main() {
  let  nums = read_ints("inputs/day1.txt");
  println!("Read {} nums", nums.len());

  part1(&nums);
  part2(&nums);

}