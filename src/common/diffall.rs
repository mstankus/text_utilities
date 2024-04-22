use std::io::{Write};
use std::fs::{File};
use std::process::{Command};
//use crate::csfunctions::*;
//use crate::cscfunctions::*;
use crate::cscmaptopower::*;

pub struct DiffVecString {
  counter : usize,
}


impl DiffVecString {
  pub fn new() -> DiffVecString {
    DiffVecString { counter : 1 }
  }
}

impl Default for DiffVecString {
  fn default() -> Self {
    Self::new()
  }
}

impl CSCMapToPower<Vec<String>> for DiffVecString {
  fn invoke_mut(&mut self,v : Vec<String>) -> Vec::<Vec::<String>> {
    self.counter += 1;
    let len = v.len();
    for i in 0..len {
      for j in (i+1)..len {
        let mut _file1 = match File::open(v[i].clone()) {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        let mut _file2 = match File::open(v[j].clone()) {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        let files = format!("failed to diff {}/{}.{}",self.counter,i,j).to_string();
        let s = Command::new("diff")
          .arg(v[i].clone())
          .arg(v[j].clone())
          .output()
          .expect(&files)
          .stdout;
        println!("{:?}",Command::new("mkdir")
            .arg(self.counter.to_string())
            .output()
            .expect("Cannot make directory")
            .stdout);
        let mut file = File::create(
            format!("{}/diff-{}-{}-cc{}.txt",self.counter,i,j,s.len())).unwrap();
        let _ = file.write(b"1st file:");
        let _ = file.write(v[i].as_bytes());
        let _ = file.write(b"\n");
        let _ = file.write(b"2nd file:");
        let _ = file.write(v[j].as_bytes());
        let _ = file.write(b"\n");
        let _ = file.write_all(&s);
      }
    }
    vec![v]
  }
}
