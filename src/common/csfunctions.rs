use std::cmp::min;
use std::path::Path;
use std::collections::HashMap;
use std::fs::*;
use std::io::{BufReader,BufRead};
use std::mem::swap;
use std::process::Command;
use multimap::MultiMap;
use chrono::DateTime;
use chrono::Utc;

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

pub fn creation_date(meta : &Metadata) -> String {
  let mut result = String::new();
  if let Ok(time) = meta.created() {
    let d = DateTime::<Utc>::from(time);
    let d = d.format("%Y-%m-%d %H:%M:%S").to_string();
    result = d;
  }
  result
}

pub struct CreationDate;
impl CSFunctionMut<(String,String),(String,String)> for CreationDate {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    let meta = metadata(pr.1.clone()).unwrap();
    let d = creation_date(&meta);
    pr.0.push_str(&d);
    pr
  }
}
impl CSSelfMapMut<(String,String)> for CreationDate{}

pub struct CharacterCount;
impl CSFunctionMut<(String,String),(String,String)> for CharacterCount {
  fn invoke(&mut self,mut pr : (String,String)) -> (String,String) {
    match metadata(pr.1.clone()) {
      Ok(meta) => {
        pr.0.push_str(&meta.len().to_string());
      },
      Err(_) => panic!("Cannot open file")
    }
    pr
  }
}

impl CSSelfMapMut<(String,String)> for CharacterCount{}


pub struct CountWordsOfLength;
impl CSFunctionMut<(String,String),(String,String)> for CountWordsOfLength {
  fn invoke(&mut self,pr : (String,String)) -> (String,String) {
    let mut h = HashMap::new();
    match File::open(pr.1.clone()) {
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
        (format!("{}{:?}",pr.0,result),pr.1)
      },
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
    match metadata(pr.1.clone()) {
       Ok(meta) => {
         if meta.is_file() { 
           pr.0.push_str(&pr.1);
         } else {
           pr.0.push_str("NOT A FILE!");
           println!("\"{}\" is not a file",pr.1);
         }
       },
       Err(_) => {
         pr.0.push_str("NOT A FILE!");
         println!("\"{}\" is not a file",pr.1);
       }
    }
    pr
  }
}
impl CSSelfMapMut<(String,String)> for IsFile {}

pub fn it_is_file(s: &str) -> bool {
  match metadata(s) {
    Ok(meta) => { meta.is_file() },
    Err(_) => false
  }
}

enum PassYamlStatus {
  NoLinesEncountered,
  WithinYaml,
  OutsideYaml
}

pub struct PassYaml {
  status : PassYamlStatus,
}

impl PassYaml {
  pub fn new() -> PassYaml {
    let status = PassYamlStatus::NoLinesEncountered;
    PassYaml { status }
  }
}

impl Default for PassYaml {
  fn default() -> Self { 
    Self::new()
  }
}

impl CSCFunction<String,String> for PassYaml {
  fn invoke(&mut self,t : String) -> Option<String> {
    match self.status {
      PassYamlStatus::NoLinesEncountered => {
        if t=="---" {
          self.status = PassYamlStatus::WithinYaml;
          None
        } else {
          self.status = PassYamlStatus::OutsideYaml;
          Some(t)
        }
      },
      PassYamlStatus::WithinYaml => {
        if t=="---" {
          self.status = PassYamlStatus::OutsideYaml;
        }
        None
      },
      PassYamlStatus::OutsideYaml => {
        Some(t)
      }
    }
  }
}

pub struct ObtainFromFile;
impl CSCFunction::<String,BufReader<File>> for ObtainFromFile {
  fn invoke(&mut self,x : String) -> Option::<BufReader::<File>> {
    match File::open(x) {
      Ok(f) => {
        Some(BufReader::new(f))
      },
      Err(_) => None
    }
  }
}

pub struct BufReaderToVec;
impl CSFunctionMut::<BufReader::<File>,Vec::<String>> for BufReaderToVec {
  fn invoke(&mut self,mut x : BufReader::<File>) -> Vec::<String> {
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
}

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

struct PassMultipleBlankLines {
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
      let path : String = Path::new(&fname).file_name().unwrap().to_str().unwrap().to_string();
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

struct ToCygPath;
impl CSFunction<String,String> for ToCygPath {
  fn invoke(&self,x : String) -> String {
    let path = Command::new("cygpath")
                 .arg("-w")
                 .arg(x.trim())
                 .output()
                 .unwrap()
                 .stdout;
    String::from_utf8(path).unwrap()
  }
}

impl CSSelfMap<String> for ToCygPath {}

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

  #[test]
  fn test_pass_yaml1() {
    let mut y = PassYaml::new();
    assert_eq!(y.invoke("---".to_string()),None);
    assert_eq!(y.invoke("abc".to_string()),None);
    assert_eq!(y.invoke("---".to_string()),None);
    assert_eq!(y.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(y.invoke("ghi".to_string()),Some("ghi".to_string()));
  }

  #[test]
  fn test_pass_yaml2() {
    let mut y = PassYaml::new();
    assert_eq!(y.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(y.invoke("ghi".to_string()),Some("ghi".to_string()));
    assert_eq!(y.invoke("---".to_string()),Some("---".to_string()));
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
