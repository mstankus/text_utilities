use std::fs::*;
use std::io::BufReader;
use std::mem::swap;
use crate::file_information::*;

pub trait CSFunctionMut<In,Out> {
  fn invoke(&mut self,x : In) -> Out;
}
pub trait CSSelfMapMut<In> : CSFunctionMut<In,In> {}

pub trait CSMapToPowerMut<In> : CSFunctionMut<In,Vec<In>> {}

pub trait CSFunction<In,Out> {
  fn invoke(&self,x : In) -> Out;
}

pub trait CSSelfMap<In> : CSFunction<In,In> {}
pub trait CSMapToPower<In> : CSFunction<In,Vec<In>> {}

pub trait CSCFunction<In,Out> {
  fn invoke(&mut self,x : In) -> Option<Out>;
}


pub struct CreateVecStringFromBlankLineSeparated {
  current: Vec<String>
}

impl CreateVecStringFromBlankLineSeparated {
  pub fn new() -> CreateVecStringFromBlankLineSeparated {
     CreateVecStringFromBlankLineSeparated { current: Vec::new() }
  }
}

impl Default for CreateVecStringFromBlankLineSeparated {
  fn default() -> Self {
      Self::new()
  }
}

impl CSCFunction<String,Vec::<String>> for CreateVecStringFromBlankLineSeparated {
  fn invoke(&mut self,s : String) -> Option::<Vec::<String>> {
    if s.is_empty() {
      let mut v = Vec::new();
      swap(&mut v,&mut self.current);
      Some(v)
    } else {
      self.current.push(s);
      None
    }
  }
}

pub struct AppendCharacter {
  c : char
}

impl AppendCharacter {
  pub fn new(c : char) -> AppendCharacter {
    AppendCharacter { c }
  }
}

impl CSFunctionMut<(String,String),(String,String)> for AppendCharacter {
  fn invoke(&mut self,mut pr : (String,String))-> (String,String) {
    pr.0.push(self.c);
    pr
  }
}
impl CSSelfMapMut<(String,String)> for AppendCharacter{}

pub struct AppendString {
  c : String
}

impl AppendString {
  pub fn new(c : String) -> AppendString {
    AppendString { c }
  }
}

impl CSFunctionMut<(String,String),(String,String)> for AppendString {
  fn invoke(&mut self,mut pr : (String,String))-> (String,String) {
    pr.0.push_str(&self.c);
    pr
  }
}

impl CSSelfMapMut<(String,String)> for AppendString{}

pub struct AppendOriginal;

impl CSFunctionMut<(String,String),(String,String)> for AppendOriginal {
  fn invoke(&mut self,mut pr : (String,String))-> (String,String) {
    pr.0.push_str(&pr.1);
    pr
  }
}
impl CSSelfMapMut<(String,String)> for AppendOriginal {}

pub struct ComposeCSFunctionMut<In : Sized> {
  data: Vec<Box<dyn CSSelfMapMut::<In>>>
}

impl<In> ComposeCSFunctionMut<In> {
  pub fn new(data : Vec<Box<dyn CSSelfMapMut::<In>>>) -> ComposeCSFunctionMut<In> {
    ComposeCSFunctionMut::<In> { data }
  }
}

impl<In> CSFunctionMut<In,In> for ComposeCSFunctionMut<In> {
  fn invoke(&mut self,mut x : In) -> In {
    for item in &mut self.data {
      x = item.invoke(x);
    }
    x
  }
}
impl<In> CSSelfMapMut<In> for ComposeCSFunctionMut<In> {}

pub struct OriginalModified {
  n1 : usize,
  n2 : usize,
}

impl CSFunctionMut<(String,String),(String,String)> for OriginalModified {
  fn invoke(&mut self,mut pr : (String,String))-> (String,String) {
    pr.0.push_str(&pr.1[self.n1..(pr.1.len()-self.n2)]);
    pr
  }
}

impl OriginalModified {
  pub fn new(n1 : usize,n2: usize) -> OriginalModified {
    OriginalModified { n1, n2 }
  }
}
impl CSSelfMapMut<(String,String)> for OriginalModified {}

pub struct CreationDate;
impl CSFunctionMut<(String,String),(String,String)> for CreationDate {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    let d = creation_date(&pr.1);
    pr.0.push_str(&d.unwrap());
    pr
  }
}
impl CSSelfMapMut<(String,String)> for CreationDate{}

pub struct CharacterCount;
impl CSFunctionMut<(String,String),(String,String)> for CharacterCount {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    match character_count(&pr.1) {
      Ok(cnt) => pr.0.push_str(&cnt.to_string()),
      Err(_) => panic!("Cannot open file")
    }
    pr
  }
}

impl CSSelfMapMut<(String,String)> for CharacterCount{}

pub struct CountWordsOfLength;
impl CSFunctionMut<(String,String),(String,String)> for CountWordsOfLength {
  fn invoke(&mut self,pr : (String,String)) -> (String,String) {
    match count_words_of_length(&pr.1) {
      Ok(pairs) => (format!("{}{:?}",pr.0,pairs),pr.1),
      Err(_) =>  {
        println!("Cannot open the file \"{}\"",pr.1);
        ("Bad file".to_string(),pr.1)
      }
    }
  }
}

impl CSSelfMapMut<(String,String)> for CountWordsOfLength {}

pub struct Counter {
  counter : usize,
}

impl Counter {
  pub fn new(counter : usize) -> Counter {
    Counter { counter }
  }
}

impl CSFunctionMut<(String,String),(String,String)> for Counter {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    pr.0.push_str(&self.counter.to_string());
    self.counter += 1;
    pr
  }
}
impl CSSelfMapMut<(String,String)> for Counter {}

pub struct IsFile;
impl CSFunctionMut<(String,String),(String,String)> for IsFile {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    let result = is_file(&pr.1);
    let flag = result.is_ok() && result.unwrap();
    if flag {
      pr.0.push_str(&pr.1);
    } else {
      pr.0.push_str("NOT A FILE!");
      println!("\"{}\" is not a file",pr.1);
    }
    pr
  }
}

impl CSSelfMapMut<(String,String)> for IsFile {}

pub struct ObtainFromFile;
impl CSCFunction::<String,BufReader<File>> for ObtainFromFile {
  fn invoke(&mut self,x : String) -> Option::<BufReader::<File>> {
    get_bufreader(&x)
  }
}

pub struct BufReaderToVec;
impl CSFunctionMut::<BufReader::<File>,Vec::<String>> for BufReaderToVec {
  fn invoke(&mut self,x : BufReader::<File>) -> Vec::<String> {
    buf_reader_to_vec(x)
  }
}

pub struct FunctionOnBunch<T> {
  b : Box::<dyn CSFunctionMut::<T,Vec::<T>>>
}

impl<T> CSFunctionMut::<Vec::<T>,Vec::<T>> for FunctionOnBunch<T> {
  fn invoke(&mut self,x : Vec<T>) -> Vec<T> {
    let mut result = Vec::<T>::new();
    for item in x {
      result.append(&mut self.b.invoke(item));
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_invoke_quote() {
    let mut it = AppendCharacter { c : '\"' };
    let orig = "def".to_string();
    assert_eq!(it.invoke(("abc".to_string(),orig.clone())),("abc\"".to_string(),orig));
  }

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
}
