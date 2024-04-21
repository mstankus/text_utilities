use std::cmp::min;
use std::path::Path;
use std::fs::*;
use multimap::MultiMap;
use crate::csfunctions::*;

pub struct RemoveLittleVecString;
impl CSCFunction::<Vec::<String>,Vec::<String>> for RemoveLittleVecString {
  fn invoke(&mut self,x : Vec::<String>) -> Option::<Vec::<String>> {
    if x.len() > 1 {
      Some(x) 
    } else {
      None
    }
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

impl CSFunctionMut<(String,String),(usize,usize,bool)> for CompareStrings {
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

impl CSCFunction<String,String> for PassMultipleBlankLines {
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
impl CSFunction::<Vec::<String>,Vec::<Vec::<String>>> for SplitByName {
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

impl CSMapToPower::<Vec::<String>> for SplitByName {}

pub struct SplitBySize;
impl CSFunction::<Vec::<String>,Vec::<Vec::<String>>> for SplitBySize {
  fn invoke(&self,x : Vec::<String>) -> Vec::<Vec::<String>> {
    let mut result = Vec::<Vec::<String>>::new();
    let mut h = MultiMap::new();
    for item in x {
      if let Ok(meta) = metadata(item.clone()) {
        let sz = meta.len();
        h.insert(sz,item.clone());
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

impl CSMapToPower::<Vec::<String>> for SplitBySize {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_original() {
    let mut it = AppendOriginal {};
    let orig = "def".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abcdef".to_string(),orig));
  }

  #[test]
  fn test_construct_append() {
    let mut it = AppendString::new("what".to_string());
    let orig = "def".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abcwhat".to_string(),orig));
  }

  #[test]
  fn test_construct_original_modified() {
    let mut it = OriginalModified::new(1,2);
    let orig = "0123456".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc1234".to_string(),orig));
  }

  #[test]
  fn test_construct_character_count() {
    let mut it = CharacterCount{};
    let orig = "test_file.txt".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc27".to_string(),orig));
  }

  #[test]
  fn test_construct_count_words_of_length() {
    let mut it = CountWordsOfLength{};
    let orig = "test_file.txt".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc[(2, 2), (3, 3), (4, 1)]".to_string(),orig));
  }

  #[test]
  fn test_construct_counter() {
    let mut it = Counter::new(3);
    let orig = "junk".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc3".to_string(),orig.clone()));
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc4".to_string(),orig.clone()));
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc5".to_string(),orig.clone()));
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abc6".to_string(),orig.clone()));
  }

  #[test]
  fn test_construct_isfile() {
    let mut it = IsFile{ };
    let orig = "junk".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abcNOT A FILE!".to_string(),orig.clone()));
    let orig = "test_file.txt".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),
               ("abctest_file.txt".to_string(),orig));
  }

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
