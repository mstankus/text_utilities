//use common::csfunctions::*;
use common::cscfunctions::*;
//use common::composecsfunctions::*;
use common::composecscfunctions::*;
use common::two_strings::*;
use std::io::{BufRead,stdin};
use std::env;

fn create_modify_file_processing(args : &Vec::<String>) 
      -> ComposeCSCSelfMap::<TwoStrings> {
  let mut vec : Vec::<Box::<dyn CSCSelfMap::<TwoStrings>>>
       = Vec::new();
  for item in args {
    if item.starts_with('-') {
      if item=="-" { continue; }
      if item=="-q" {
        vec.push(Box::new(AppendCharacter::new('\"')));
      } else if item=="-o" {
        vec.push(Box::new(AppendOriginal{}));
      } else if let Some(nums) =item.strip_prefix("-o=") {
        let mut iter = nums.split(',');
        let n1 = iter.next().unwrap().parse::<usize>().unwrap();
        let n2 = iter.next().unwrap().parse::<usize>().unwrap();
        vec.push(Box::new(OriginalModified::new(n1,n2)));
      } else if let Some(str) =item.strip_prefix("-a=") {
        vec.push(Box::new(AppendString::new(str.to_string())));
      } else if item=="-date" {
        vec.push(Box::new(CreationDate{}));
      } else if item=="-cc" {
        vec.push(Box::new(CharacterCount{}));
      } else if item=="-count_words" {
        vec.push(Box::new(CountWordsOfLength{}));
      } else if item=="-is_file" {
        vec.push(Box::new(IsFile{}));
      } else if item=="-counter" {
        vec.push(Box::new(Counter::new(1)));
      } else {
        panic!("Cannot find the option {}",item);
      }
    }
  }
  ComposeCSCSelfMap::<TwoStrings>::new(vec)
}

fn create_modify_file_vec(args : &[String]) -> Vec::<String> {
  let mut result : Vec::<String> = vec![]; 
  for item in args.iter().skip(1) {
    if item=="-" {
      let mut to_add : Vec<String> = 
         stdin()
           .lock()
           .lines()
           .map(|x| x.unwrap().trim().to_string())
           .collect();
      result.append(&mut to_add);
    } else if !item.starts_with('-') {
      result.push(item.clone());
    }
  }
  result
}

fn main() {
  let args = env::args().collect::<Vec<_>>();
  let mut processing = create_modify_file_processing(&args);
  let fns = create_modify_file_vec(&args);
  for a_filename in fns {  
    let mut pr = TwoStrings::new(a_filename.clone());
    pr = processing.invoke(pr);
    println!("{}",pr.answer);
  }
}

/*
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_modifyfile_obtain() {
    let vec = ["-a=add".to_string(),
               "hi".to_string(),
               "there".to_string()];
    let mut it = create_modify_file_vec(&vec);
    assert_eq!(it.obtain(),Some("hi".to_string()));
    assert_eq!(it.obtain(),Some("there".to_string()));
    assert_eq!(it.obtain(),None);
  }
}
*/
