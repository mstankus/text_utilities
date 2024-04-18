use common::csfunctions::*;
use std::io::{BufRead,stdin};
use std::env;

fn create_multidiff_processing(args : &Vec::<String>) 
  -> Vec::<Box::<dyn CSMapToPower::<Vec::<String>>>>{
  let mut vec : Vec::<Box::<dyn CSMapToPower::<Vec::<String>>>>= Vec::new();
  for item in args {
    if item.starts_with('-') {
      if item=="-" { continue; }
      if item=="-n" {
        //println!("setting up -n");
        vec.push(Box::new(SplitByName{}));
      } else if item=="-s" {
        //println!("setting up -s");
        vec.push(Box::new(SplitBySize{}));
/*
      } else if item=="-d" {
        //println!("setting up -d");
        vec.push(Box::new(SplitByCompare{}));
*/
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

fn main() {
  let args = env::args().collect::<Vec<_>>();
  //println!("args:{:?}",args);
  let processing = create_multidiff_processing(&args);
  let mut fns = create_multidiff_vec(&args);
  fns.push("".to_string());
  //println!("fns:{:?}",fns);
  let mut blocker = CreateVecStringFromBlankLineSeparated::new();
  for a_filename in fns {  
    if let Some(block) = blocker.invoke(a_filename) {
      //println!("block:{:?}",block);
      let mut old_blocks = vec![block];
      for it in &processing {
        let mut new_blocks = vec![];
        for blk in old_blocks {
          let mut blocks = it.invoke(blk);
          new_blocks.append(&mut blocks);
        }
        old_blocks = new_blocks;
      }
      let result = old_blocks;
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
/*
#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn test_create_modifyfile_obtain() {
    let vec = ["-a=add".to_string(),
               "hi".to_string(),
               "there".to_string()];
    let mut it = create_modify_file_obtain(&vec);
    assert_eq!(it.obtain(),Some("hi".to_string()));
    assert_eq!(it.obtain(),Some("there".to_string()));
    assert_eq!(it.obtain(),None);
  }
}
*/
