//use std::mem::swap;
use std::io::BufReader;
use std::fs::File;
use crate::file_information::*;

pub trait CSExtFunction<In,Out> {
  fn invoke(&self,x : In) -> Option<Out>;
}

pub trait CSExtSelfMap<In> {
  fn invoke(&self,x : In) -> Option::<In>;
}

pub trait CSCExtFunction<In,Out> {
  fn invoke(&mut self,x : In) -> Option::<Out>;
}

pub trait CSCExtSelfMap<In> {
  fn invoke(&mut self,x : In) -> Option<In>;
}

pub struct ObtainFromFile;
impl CSCExtFunction::<String,BufReader<File>> for ObtainFromFile {
  fn invoke(&mut self,x : String) -> Option::<BufReader::<File>> {
    get_bufreader(&x)
  }
}

/*
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
*/
