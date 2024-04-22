use file_diff::{diff_files};
use std::cmp::min;
use std::fs::{File};
use std::path::Path;
use std::fs::*;
use multimap::MultiMap;
//use crate::csfunctions::*;
use crate::csmaptopower::*;
use crate::cscmaptopower::*;
use crate::cscfunctions::*;
use crate::csextfunctions::*;
//use crate::two_strings::*;


pub struct RemoveLittleVecString;
impl CSMapToPower::<Vec::<String>> for RemoveLittleVecString {
  fn invoke(&self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    let mut result = vec![];
    if x.len() > 1 {
      result.push(x);
    }
    result
  } 
}

impl CSCMapToPower::<Vec::<String>> for RemoveLittleVecString {
  fn invoke_mut(&mut self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    self.invoke(x)
  } 
}

pub struct CompareStrings {
  n1 : usize,
  n2 : usize,
}

impl CompareStrings {
  pub fn new(n1 : usize, n2 : usize) -> CompareStrings {
    CompareStrings { n1, n2 }
  }
}

impl CSCFunction<(String,String),(usize,usize,bool)> for CompareStrings {
  fn invoke(&mut self,x : (String,String)) -> (usize,usize,bool) {
    let they_are_equal = x.0==x.1;
    if they_are_equal {
      self.n1 += x.0.len();
      self.n2 += x.1.len();
    } else {
      let mut w1 = x.0.chars();
      let mut w2 = x.1.chars();
      let mut k = 0;
      for _ in 0..min(x.0.len(),x.1.len()) {
        if w1.next()==w2.next() { k += 1; }
      }
      self.n1 += k;
      self.n2 += k;
    }
    (self.n1,self.n2,they_are_equal)
  }
}

pub struct PassMultipleBlankLines {
  previous_line : Option<String>,
}

impl CSCExtFunction<String,String> for PassMultipleBlankLines {
  fn invoke(&mut self,x : String) -> Option<String> {
    let use_none = x.is_empty() && 
                   self.previous_line.is_some() && 
                   self.previous_line.as_ref().unwrap().is_empty();
    if use_none {
      None
    } else {
      self.previous_line = Some(x.clone());
      Some(x)
    } 
  }
}

pub struct SplitByName;
impl CSMapToPower::<Vec::<String>> for SplitByName {
  fn invoke(&self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    let mut result = Vec::<Vec::<String>>::new();
    let mut h = MultiMap::new();
    for item in x {
      let fname = item.clone();
      let path : String = Path::new(&fname)
          .file_name().unwrap()
          .to_str().unwrap().to_string();
      h.insert(path.clone(),item.clone());
    }
    //println!("{:?}",h);
    for (_,values) in h.iter_all() {
      result.push(values.to_vec());
    }
    //println!("{:?}",result);
    result
  }
}

impl CSCMapToPower::<Vec::<String>> for SplitByName {
  fn invoke_mut(&mut self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    self.invoke(x)
  } 
}

pub struct SplitBySize;
impl CSMapToPower::<Vec::<String>> for SplitBySize {
  fn invoke(&self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    let mut result = Vec::<Vec::<String>>::new();
    let mut h = MultiMap::new();
    for item in x {
      if let Ok(meta) = metadata(item.clone()) {
        let sz = meta.len();
        println!("sz1:{}",sz);
        h.insert(sz,item.clone());
        println!("item1:{:?}",item.clone());
      }
    }
    //println!("{:?}",h);
    for (_,values) in h.iter_all() {
      result.push(values.to_vec());
    }
    //println!("{:?}",result);
    result
  }
}

impl CSCMapToPower::<Vec::<String>> for SplitBySize{
  fn invoke_mut(&mut self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    self.invoke(x)
  } 
}

pub struct SplitByCompare;
impl CSMapToPower::<Vec::<String>> for SplitByCompare {
  fn invoke(&self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    let mut todo = x;
    let mut klasses = vec![];
    while let Some(it) = todo.pop() {
      let mut yes = vec![it.clone()];
      let mut no = vec![];
      while let Some(another) = todo.pop() {
        let mut file1 = match File::open(it.clone()) {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open(another.clone()) {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        //println!("diffing {} with {}",it,another);
        if diff_files(&mut file1, &mut file2) {
          yes.push(another.clone());
        } else {
          no.push(another.clone());
        }
      }
      //println!("yes:{:?}",yes);
      //println!("no:{:?}",no);
      klasses.push(yes);
      todo = no;
    }
    klasses
  }
}

impl CSCMapToPower::<Vec::<String>> for SplitByCompare {
  fn invoke_mut(&mut self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    self.invoke(x)
  } 
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn compare_strings_1() {
    let mut compare = CompareStrings::new(0,0);
    assert_eq!(compare.invoke(("abc".to_string(),"abc".to_string())),
               (3,3,true));
    assert_eq!(compare.invoke(("abc".to_string(),"abd".to_string())),
               (5,5,false));
  }

  #[test]
  fn compare_strings_2() {
    let mut compare = CompareStrings::new(10,20);
    assert_eq!(compare.invoke(("abc".to_string(),"abc".to_string())),
               (13,23,true));
    assert_eq!(compare.invoke(("abc".to_string(),"abd".to_string())),
               (15,25,false));
  }

  #[test]
  fn compare_strings_3() {
    let mut compare = CompareStrings::new(10,20);
    assert_eq!(compare.invoke(("abde".to_string(),"abc".to_string())),
               (12,22,false));
    assert_eq!(compare.invoke(("abc".to_string(),"abc".to_string())),
               (15,25,true));
  }
 
  #[test]
  fn test_pass_multiple_blank_lines_1() {
    let mut pass = PassMultipleBlankLines { previous_line : None };
    assert_eq!(pass.invoke("abc".to_string()),Some("abc".to_string()));
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
  }

  #[test]
  fn test_pass_multiple_blank_lines_2() {
    let mut pass = PassMultipleBlankLines { previous_line : None };
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("abc".to_string()),Some("abc".to_string()));
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("".to_string()),None);
  }

  #[test]
  fn test_pass_multiple_blank_lines_3() {
    let mut pass = PassMultipleBlankLines { previous_line : None };
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("abc".to_string()),Some("abc".to_string()));
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(pass.invoke("".to_string()),Some("".to_string()));
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("".to_string()),None);
    assert_eq!(pass.invoke("def".to_string()),Some("def".to_string()));
  }
}
