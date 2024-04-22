//use common::composecscfunctions::*;
use common::csextfunctions::*;
//use common::csmaptopower::*;
use common::cscmaptopower::*;
use common::diffall::*;
use common::only_multidiff::*;
use common::string_to_vecstring::*;
//use common::two_strings::*;
use std::io::{BufRead,stdin};
use std::env;

fn create_multidiff_processing(args : &Vec::<String>) 
  -> Vec::<Box::<dyn CSCMapToPower::<Vec::<String>>>>{
  let mut vec : Vec::<Box::<dyn CSCMapToPower::<Vec::<String>>>>= Vec::new();
  for item in args {
    if item.starts_with('-') {
      if item=="-" { continue; }
      if item=="-n" {
        //println!("setting up -n");
        vec.push(Box::new(SplitByName{}));
      } else if item=="-s" {
        //println!("setting up -s");
        vec.push(Box::new(SplitBySize{}));
      } else if item=="-d" {
        //println!("setting up -d");
        vec.push(Box::new(SplitByCompare{}));
      } else if item=="-x" {
        //println!("setting up -x");
        vec.push(Box::new(RemoveLittleVecString{}));
      } else if item=="-diff" {
        //println!("setting up -diff");
        vec.push(Box::new(DiffVecString::new()));
      } else {
        panic!("Cannot find the option {}",item);
      }
    }
  }
  vec
}

fn create_multidiff_vec(args : &[String]) -> Vec::<String> {
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
  //println!("The files are {:?}",result);
  result
}

fn work_through_arguments(args : Vec<String>) ->
  (Vec::<Box::<dyn CSCMapToPower<Vec<String>>>>,Vec<String>) {
  let processing = create_multidiff_processing(&args);
  //println!("args:{:?}",args);
  let fns = create_multidiff_vec(&args);
  (processing,fns)
}

fn work_with_block(
    processing :
    &mut Vec::<Box::<dyn CSCMapToPower::<Vec<String>>>>,
    block : Vec::<String>) -> Vec::<Vec::<String>> 
{
  let mut old_blocks = vec![block];
  for it in processing {
    let mut new_blocks = vec![];
    for blk in old_blocks {
      let mut blocks = it.invoke_mut(blk);

      new_blocks.append(&mut blocks);
    }
    old_blocks = new_blocks;
  }
  old_blocks
}

fn main() {
  let args = env::args().collect::<Vec<_>>();
  //println!("args:{:?}",args);
  let (mut processing,mut fns) = work_through_arguments(args);
  fns.push("".to_string());
  //println!("fns:{:?}",fns);
  let mut blocker = CreateVecStringFromBlankLineSeparated::new();
  for a_filename in fns {  
    if let Some(block) = blocker.invoke(a_filename) {
      let result = work_with_block(&mut processing,block);
      //println!("result:{:?}",result);
      for a_block in result {
        for a_file in a_block {
          println!("{}",a_file);
        }
        println!();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_multidiff_1() {
    let args = vec![
        "-n".to_string(),
        "a".to_string(),
        "b".to_string(),
        "x/a".to_string(),
        "x/b".to_string()
    ];
    let (mut processing,fns) = work_through_arguments(args);
    let mut result = work_with_block(&mut processing,fns);
    for item in result.iter_mut() {
      item.sort();
    }
    result.sort();
    let answer = vec![
           vec!["a".to_string(),"x/a".to_string()],
           vec!["b".to_string(),"x/b".to_string()],
    ];
    assert_eq!(result,answer);
  }

  #[test]
  fn test_multidiff_2() {
    let args = vec![
        "-s".to_string(),
        "test_data/a".to_string(),
        "test_data/b".to_string(),
        "test_data/x/a".to_string(),
        "test_data/x/b".to_string()
    ];
    let (mut processing,fns) = work_through_arguments(args);
    assert_eq!(fns,vec![
         "test_data/a".to_string(),
         "test_data/b".to_string(),
         "test_data/x/a".to_string(),
         "test_data/x/b".to_string(),
    ]);
    let mut result = work_with_block(&mut processing,fns);
    //println!("result1:{:?}",result);
    result.sort();
    let answer = vec![
           vec!["test_data/a".to_string(),
                "test_data/x/a".to_string(),
                "test_data/x/b".to_string(),
           ],
           vec!["test_data/b".to_string()],
    ];
    assert_eq!(result,answer);
  }

  #[test]
  fn test_multidiff_3() {
    let args = vec![
        "-d".to_string(),
        "test_data/a".to_string(),
        "test_data/b".to_string(),
        "test_data/x/a".to_string(),
        "test_data/x/b".to_string(),
        "test_data/x/c".to_string()
    ];
    let (mut processing,fns) = work_through_arguments(args);
    assert_eq!(fns,vec![
         "test_data/a".to_string(),
         "test_data/b".to_string(),
         "test_data/x/a".to_string(),
         "test_data/x/b".to_string(),
         "test_data/x/c".to_string(),
    ]);
    let mut result = work_with_block(&mut processing,fns);
    //println!("result1:{:?}",result);
    result.sort();
    let answer = vec![
           vec![
             "test_data/a".to_string(),
             "test_data/x/a".to_string(),
             "test_data/x/b".to_string(),
           ],
           vec![
             "test_data/b".to_string(),
           ],
           vec![
             "test_data/x/c".to_string(),
           ],
    ];
    assert_eq!(result,answer);
  }

  #[test]
  fn test_multidiff_4() {
    let args = vec![
        "-d".to_string(),
        "-x".to_string(),
        "test_data/a".to_string(),
        "test_data/b".to_string(),
        "test_data/x/a".to_string(),
        "test_data/x/b".to_string(),
        "test_data/x/c".to_string()
    ];
    let (mut processing,fns) = work_through_arguments(args);
    assert_eq!(fns,vec![
         "test_data/a".to_string(),
         "test_data/b".to_string(),
         "test_data/x/a".to_string(),
         "test_data/x/b".to_string(),
         "test_data/x/c".to_string(),
    ]);
    let mut result = work_with_block(&mut processing,fns);
    //println!("result1:{:?}",result);
    result.sort();
    let answer = vec![
           vec![
             "test_data/a".to_string(),
             "test_data/x/a".to_string(),
             "test_data/x/b".to_string(),
           ],
    ];
    assert_eq!(result,answer);
  }
  #[test]
  fn test_multidiff_5() {
    let args = vec![
        "-diff".to_string(),
        "test_data/a".to_string(),
        "test_data/b".to_string(),
        "test_data/x/a".to_string(),
        "test_data/x/b".to_string(),
        "test_data/x/c".to_string()
    ];
    let (mut processing,fns) = work_through_arguments(args);
    assert_eq!(fns,vec![
         "test_data/a".to_string(),
         "test_data/b".to_string(),
         "test_data/x/a".to_string(),
         "test_data/x/b".to_string(),
         "test_data/x/c".to_string(),
    ]);
    let _ = work_with_block(&mut processing,fns);
  }
}
