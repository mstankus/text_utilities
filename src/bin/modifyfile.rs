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

fn work_through_arguments(args : Vec::<String>) -> 
(ComposeCSCSelfMap<TwoStrings>,Vec::<String>) {
  let processing = create_modify_file_processing(&args);
  let fns = create_modify_file_vec(&args);
  (processing,fns)
}

fn process_one_string(processing: &mut ComposeCSCSelfMap<TwoStrings>,a_filename : &str) -> String {
  let mut pr = TwoStrings::new(a_filename.to_string());
  pr = processing.invoke(pr);
  pr.answer
}

fn main() { 
  let (mut processing,fns) = work_through_arguments(env::args().collect::<Vec<_>>());
  for a_filename in fns {  
    println!("{}",process_one_string(&mut processing,&a_filename));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_modifyfile_obtain_1() {
    let vec = vec!["-a=add".to_string(),
               "hi".to_string(),
               "there".to_string()];
    let (mut processing,fns) = work_through_arguments(vec);
    assert_eq!(process_one_string(&mut processing,&fns[0]),"add".to_string());
    assert_eq!(process_one_string(&mut processing,&fns[1]),"add".to_string());
  }

  #[test]
  fn test_create_modifyfile_obtain_2() {
    let vec = vec!["-o".to_string(),
                   "hi".to_string(),
                   "there".to_string()];
    let (mut processing,fns) = work_through_arguments(vec);
    assert_eq!(process_one_string(&mut processing,&fns[0]),"hi".to_string());
    assert_eq!(process_one_string(&mut processing,&fns[1]),"there".to_string());
  }

  #[test]
  fn test_create_modifyfile_obtain_3() {
    let vec = vec![ "-a=dude".to_string(),
                    "-o".to_string(),
                   "hi".to_string(),
                   "there".to_string()];
    let (mut processing,fns) = work_through_arguments(vec);
    assert_eq!(process_one_string(&mut processing,&fns[0]),"dudehi".to_string());
    assert_eq!(process_one_string(&mut processing,&fns[1]),"dudethere".to_string());
  }

  #[test]
  fn test_create_modifyfile_obtain_4() {
    let vec = vec![ "-a=dude".to_string(),
                    "-o".to_string(),
                    "-a=yo".to_string(),
                   "hi".to_string(),
                   "there".to_string()];
    let (mut processing,fns) = work_through_arguments(vec);
    assert_eq!(process_one_string(&mut processing,&fns[0]),"dudehiyo".to_string());
    assert_eq!(process_one_string(&mut processing,&fns[1]),"dudethereyo".to_string());
  }
}
