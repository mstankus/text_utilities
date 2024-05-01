//use std::fs::*;
//use std::io::BufReader;
//use crate::csfunctions::*;
use crate::cscfunctions::*;
use crate::file_information::*;

#[derive(PartialEq,Debug)]
pub struct TwoStrings {
  pub answer : String,
  original   : String,
}

impl TwoStrings {
  pub fn new(original : String) -> TwoStrings {
    TwoStrings { answer: String::new(),original }
  }
  pub fn new_with(answer: String,original : String) -> TwoStrings {
    TwoStrings { answer,original }
  }
}

pub struct AppendCharacter { c : char }

impl AppendCharacter {
  pub fn new(c: char) -> AppendCharacter {
    AppendCharacter { c }
  }
}

impl CSCSelfMap<TwoStrings> for AppendCharacter {
  fn invoke(&mut self,mut pr : TwoStrings) -> TwoStrings {
    pr.answer.push(self.c);
    pr
  }
}

pub struct AppendString {
  c : String
}

impl AppendString {
  pub fn new(c : String) -> AppendString {
    AppendString { c }
  }
}

impl CSCSelfMap<TwoStrings> for AppendString {
  fn invoke(&mut self,mut pr : TwoStrings)-> TwoStrings {
    pr.answer.push_str(&self.c);
    pr
  }
}

//impl CSCSelfMap<TwoStrings> for AppendString{}

pub struct AppendOriginal;

impl CSCSelfMap<TwoStrings> for AppendOriginal {
  fn invoke(&mut self,mut pr : TwoStrings)-> TwoStrings {
    pr.answer.push_str(&pr.original);
    pr
  }
}
//impl CSCSelfMap<(String,String)> for AppendOriginal {}

pub struct OriginalModified {
  n1 : usize,
  n2 : usize,
}

impl CSCSelfMap<TwoStrings> for OriginalModified {
  fn invoke(&mut self,mut pr : TwoStrings)-> TwoStrings {
    pr.answer.push_str(&pr.original[self.n1..(pr.original.len()-self.n2)]);
    pr
  }
}

impl OriginalModified {
  pub fn new(n1 : usize,n2: usize) -> OriginalModified {
    OriginalModified { n1, n2 }
  }
}
pub struct CreationDate;
impl CSCSelfMap<TwoStrings> for CreationDate {
  fn invoke(&mut self,mut pr : TwoStrings) -> TwoStrings {
    let d = creation_date(&pr.original);
    pr.answer.push_str(&d.unwrap());
    pr
  }
}

pub struct CharacterCount;
impl CSCSelfMap<TwoStrings> for CharacterCount {
  fn invoke(&mut self,mut pr : TwoStrings) -> TwoStrings {
    match character_count(&pr.original) {
      Ok(cnt) => pr.answer.push_str(&cnt.to_string()),
      Err(_) => panic!("Cannot open file")
    }
    pr
  }
}

pub struct CountWordsOfLength;
impl CSCSelfMap<TwoStrings> for CountWordsOfLength {
  fn invoke(&mut self,pr : TwoStrings) -> TwoStrings {
    match count_words_of_length(&pr.original) {
      Ok(pairs) => {
        TwoStrings::new_with(
            format!("{}{:?}",pr.answer,pairs),
            pr.original)
      }
      Err(_) =>  {
        println!("Cannot open the file \"{}\"",pr.original);
        TwoStrings::new_with("Bad file".to_string(),pr.original)
      }
    }
  }
}

pub struct IsFile;
impl CSCSelfMap<TwoStrings> for IsFile {
  fn invoke(&mut self,mut pr : TwoStrings) -> TwoStrings {
    let result = is_file(&pr.original);
    let flag = result.is_ok() && result.unwrap();
    if flag {
      pr.answer.push_str(&pr.original);
    } else {
      pr.answer.push_str("NOT A FILE!");
      println!("\"{}\" is not a file",pr.original);
    }
    pr
  }
}

pub struct FunctionOnBunch<T> {
  b : Box::<dyn CSCFunction::<T,Vec::<T>>>
}

impl<T> CSCFunction::<Vec::<T>,Vec::<T>> for FunctionOnBunch<T> {
  fn invoke(&mut self,x : Vec<T>) -> Vec<T> {
    let mut result = Vec::<T>::new();
    for item in x {
      result.append(&mut self.b.invoke(item));
    }
    result
  }
}

pub struct Counter {
  counter : usize,
}

impl Counter {
  pub fn new(counter : usize) -> Counter {
    Counter { counter }
  }
}

impl CSCSelfMap<TwoStrings> for Counter {
  fn invoke(&mut self,mut pr : TwoStrings) -> TwoStrings {
    pr.answer.push_str(&self.counter.to_string());
    self.counter += 1;
    pr
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct_counter() {
    let mut it = Counter::new(3);

    let start = TwoStrings::new_with("abc".to_string(),"junk".to_string());
    let finish1 = TwoStrings::new_with("abc3".to_string(),"junk".to_string());
    assert_eq!(it.invoke(start),finish1);

    let start = TwoStrings::new_with("abc".to_string(),"junk".to_string());
    let finish2 = TwoStrings::new_with("abc4".to_string(),"junk".to_string());
    assert_eq!(it.invoke(start),finish2);
  }

  #[test]
  fn test_invoke_quote() {
    let mut it = AppendCharacter { c : '\"' };
    let two = TwoStrings::new_with("abc".to_string(),"def".to_string());
    assert_eq!(it.invoke(two),TwoStrings::new_with("abc\"".to_string(),"def".to_string()));
  }

  #[test]
  fn test_construct_original() {
    let mut it = AppendOriginal {};
    let start = TwoStrings::new_with("abc".to_string(),"def".to_string());
    let finish = TwoStrings::new_with("abcdef".to_string(),"def".to_string());
    assert_eq!(it.invoke(start),finish);
  }

  #[test]
  fn test_construct_append() {
    let mut it = AppendString::new("what".to_string());
    let start = TwoStrings::new_with("abc".to_string(),"def".to_string());
    let finish = TwoStrings::new_with("abcwhat".to_string(),"def".to_string());
    assert_eq!(it.invoke(start),finish);
  }

  #[test]
  fn test_construct_original_modified() {
    let mut it = OriginalModified::new(1,2);
    let start = TwoStrings::new_with("abc".to_string(),"0123456".to_string());
    let finish = TwoStrings::new_with("abc1234".to_string(),"0123456".to_string());
    assert_eq!(it.invoke(start),finish);
  }

  #[test]
  fn test_construct_character_count() {
    let mut it = CharacterCount{};
    let start = TwoStrings::new_with("abc".to_string(),"test_data/test_file.txt".to_string());
    let finish = TwoStrings::new_with("abc23".to_string(),"test_data/test_file.txt".to_string());
    assert_eq!(it.invoke(start),finish);
  }

  #[test]
  fn test_construct_count_words_of_length() {
    let mut it = CountWordsOfLength{};
    let start = TwoStrings::new_with("abc".to_string(),"test_data/test_file.txt".to_string());
    let finish = TwoStrings::new_with("abc[(2, 2), (3, 3), (4, 1)]".to_string(),"test_data/test_file.txt".to_string());
    assert_eq!(it.invoke(start),finish);
  }

  #[test]
  fn test_construct_isfile() {
    let mut it = IsFile{ };
    let start = TwoStrings::new_with("abc".to_string(),"junk".to_string());
    let finish = TwoStrings::new_with("abcNOT A FILE!".to_string(),"junk".to_string());
    assert_eq!(it.invoke(start),finish);
  }
}
