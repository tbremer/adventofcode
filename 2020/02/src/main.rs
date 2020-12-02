use utils;

#[derive(Debug)]
struct Password {
  min: u16,
  max: u16,
  needle: char,
  password: String,
}

fn pluck_n_char(string: &str, n: usize) -> char {
  match string.chars().nth(n) {
    Some(v) => v,
    None => panic!("could not get {} from {}", n, string)
  }
}

fn str_to_password(input: &str) -> Password {
  let mut splits = input.split(' ');
  let min_max = match splits.next() {
    Some(v) => v,
    None => panic!("minMax not found"),
  };
  let needle = match splits.next() {
    Some(v) => pluck_n_char(v, 0),
    None => panic!("needle not found"),
  }.to_owned();
  let password = match splits.next() {
    Some(v) => v,
    None => panic!("password] not found"),
  }.to_owned();

  // YIKES
  let mut min_max_iter = min_max.split('-');
  let min = min_max_iter.next().unwrap().parse::<u16>().unwrap();
  let max = min_max_iter.next().unwrap().parse::<u16>().unwrap();

  Password { min, max, needle, password }
}

fn is_password_valid_count(pw: &Password) -> bool {
  if pw.password.contains(&pw.needle.to_string()) == false {return false;}
  let mut count = 0;

  for ch in pw.password.chars() {
    if ch == pw.needle {
      count += 1;
    }
  }

  count >= pw.min && count <= pw.max
}

fn is_password_valid_position(pw: &Password) -> bool {
  if pw.password.contains(&pw.needle.to_string()) == false {return false;}

  let password = pw.password.clone();
  let chars: Vec<char> = password.chars().collect();
  let min = usize::from(pw.min - 1);
  let max = usize::from(pw.max - 1);

  // println!("{:?}", pw);

  if chars[min] != pw.needle && chars[max] != pw.needle || chars[min]== pw.needle && chars[max] == pw.needle { return false; }
  

  true
}

fn str_to_pw_vec(string: &str) -> Vec<Password> {
  string.split("\n").map(str_to_password).collect()
}

fn main() {
  let test_input= "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
  let test_pw = str_to_pw_vec(test_input);
  let input_data = utils::read_file(utils::args().remove(0));
  let input_pw = str_to_pw_vec(&input_data);

  let p1_test = test_pw.iter().filter(|pw| is_password_valid_count(pw));
  let p1_assertion = input_pw.iter().filter(|pw| is_password_valid_count(pw));

  println!("test data: {}.", p1_test.count());
  println!("pt1 count: {:?}.", p1_assertion.count());

  let p2_test = test_pw.iter().filter(|pw| is_password_valid_position(pw));
  let p2_assertion = input_pw.iter().filter(|pw| is_password_valid_position(pw));

  println!("part 2 test data: {}.", p2_test.count());
  println!("part 2 data: {}.", p2_assertion.count());

  // let  p2_test = 

}
