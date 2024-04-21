use std::mem::swap;
//use std::io::BufReader;
//use std::fs::File;
//use crate::file_information::*;
use crate::csextfunctions::*;

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

impl CSCExtFunction<String,Vec::<String>> for CreateVecStringFromBlankLineSeparated {
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
