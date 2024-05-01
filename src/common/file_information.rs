use std::collections::HashMap;
use std::fs::*;
use std::io::{BufReader,BufRead};
use chrono::DateTime;
use chrono::Utc;

pub fn creation_date(s : &str) -> Result<String,std::io::Error> {
  match metadata(s) {
    Ok(meta) => {
      let time = meta.created().unwrap();
      let result = DateTime::<Utc>::from(time);
      let result = result.format("%Y-%m-%d %H:%M:%S").to_string();
      Ok(result)
    },
    Err(x) => Err(x)
  }
}

pub fn character_count(s : &str) -> Result<usize,std::io::Error> {
  match metadata(s) {
    Ok(meta) => Ok(meta.len().try_into().unwrap()),
    Err(x) => Err(x)
  }
}

pub fn count_words_of_length(s : &str) -> Result::<Vec::<(usize,usize)>,std::io::Error> {
  let mut h = HashMap::new();
  match File::open(s) {
    Ok(file) => {
      let reader = BufReader::new(file);
      for line in reader.lines() {
        //println!("line:{:?}",line);
        for it in line.unwrap().split(' ') {
          let len = it.len();
          match h.get_mut(&len) {
            Some(val) => { *val += 1; }
            ,
            None => { h.insert(len,1);}
          }
        }
      }
      let mut result : Vec<_> = h.iter().collect();
      result.sort();
      let mut pairs : Vec::<(usize,usize)> = Vec::new();
      for i in result {
        pairs.push((*i.0,*i.1));
      }
      Ok(pairs)
    },
    Err(x) => Err(x)
  }
}

pub fn is_file(s : &str) -> Result<bool,std::io::Error> {
  match metadata(s) {
    Ok(meta) => Ok(meta.is_file()),
    Err(x) => Err(x)
  }
}

pub fn get_bufreader(s : &str) -> Option::<BufReader::<File>> {
  match File::open(s) {
    Ok(f) => {
      Some(BufReader::new(f))
    },
    Err(_) => None
  }
}

pub fn buf_reader_to_vec(mut x : BufReader::<File>) -> Vec::<String> {
  let mut result = Vec::new();
  loop {
    let mut str = String::new();
    match x.read_line(&mut str) {
      Ok(n) => {
        if n==0 { 
          break;
        } else {
          result.push(str.trim().to_string());
        }
      },
      Err(_) => panic!("Can't read via buffer!")
    }
  }
  result
}

pub fn file_size(s : & str) -> Option::<u64> {
  match metadata(s) {
    Ok(meta) => Some(meta.len()),
    Err(_) => None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_character_count() {
    let result = character_count("test_data/test_file.txt");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(),23);
  }
  #[test]
  fn test_construct_count_words_of_length() {
    let result = count_words_of_length("test_data/test_file.txt");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(),vec![(2,2),(3,3),(4,1)]);
  }

  #[test]
  fn test_construct_isfile() {
    let result = is_file("junk.txt");
    assert!(result.is_err());
    let result = is_file("test_data/test_file.txt");
    assert!(result.is_ok());
    assert!(result.unwrap());
    let result = is_file("src");
    assert!(result.is_ok());
    assert!(!result.unwrap());
  }
}
