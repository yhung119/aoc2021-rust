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

fn process_bits(input: &Vec<String>) -> Vec<i32> {
  let array_len: usize = input[0].len();
  let mut common_bits: Vec<i32> =  Vec::with_capacity(array_len);
  for _ in 0..array_len {
    common_bits.push(0);
  }

  for bits in input {
    for (index, bit) in bits.chars().enumerate() {
      match bit {
        '0' => common_bits[index] -= 1,
        '1' => common_bits[index] += 1,
        _ => println!("Err")
      }
    }
  }
  common_bits
}

fn part1(input: &Vec<String>) {
  let mut common_bits = process_bits(input);

  let mut gamma = 0;
  let mut epislon = 0;
  common_bits.reverse();
  for (index, bit) in common_bits.iter().enumerate() {
    if *bit > 0 {
      gamma += i32::pow(2, index as u32);
    } else {
      epislon += i32::pow(2, index as u32);
    }
  }
  println!("gamma: {}, epislon: {}, result: {} ", gamma, epislon, gamma*epislon);
  
}


fn get_oxygen_generator_rating(input: &mut Vec<String>, position: usize) -> isize {
  if input.len() == 1 {
    return isize::from_str_radix(&input[0], 2).unwrap();
  }
  
  let common_bits = process_bits(input);

  let is_positive: char = if common_bits[position] >= 0 { '1' } else { '0' };
  let mut remaining_input: Vec<String> = Vec::new();
  for bits in input {
    if bits.chars().nth(position).unwrap() == is_positive {
      remaining_input.push(bits.to_string());
    }
  }

  return get_oxygen_generator_rating(&mut remaining_input, position+1);
}

fn get_co2_scrubber_rating(input: &mut Vec<String>, position: usize) -> isize {
  if input.len() == 1 {
    return isize::from_str_radix(&input[0], 2).unwrap();
  }
  
  let common_bits = process_bits(input);

  let is_positive: char = if common_bits[position] >= 0 { '0' } else { '1' };
  let mut remaining_input: Vec<String> = Vec::new();
  for bits in input {
    if bits.chars().nth(position).unwrap() == is_positive {
      remaining_input.push(bits.to_string());
    }
  }

  return get_co2_scrubber_rating(&mut remaining_input, position+1);
}

fn part2(input: Vec<String>) {
  let mut new_vec = input.clone();

  let oxygen_generator_rating = get_oxygen_generator_rating(&mut new_vec, 0);
  let co2_scrubber_rating = get_co2_scrubber_rating(&mut new_vec, 0);
  println!("result: {}", oxygen_generator_rating*co2_scrubber_rating);
}

fn main() {
  let input = read_input("inputs/day3.txt");
  println!("Readsss {} input", input.len());

  part1(&input);
  part2(input);

}