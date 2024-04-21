use std::io::{Write};
use std::fs::{File};
use std::process::{Command};
use crate::csfunctions::*;

pub struct DiffVecString {
  counter : usize
}


impl DiffVecString {
  pub fn new() -> DiffVecString {
    DiffVecString { counter : 0 }
  }
}

impl Default for DiffVecString {
  fn default() -> DiffVecString {
    Self::new()
  }
}
impl CSCFunction<Vec<String>,()> for DiffVecString {
  fn invoke(&mut self,v : Vec<String>) ->Option<()> {
    self.counter += 1;
    let len = v.len();
    for i in 0..len {
      for j in (i+1)..len {
        let mut _file1 = match File::open("./src/lib.rs") {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        let mut _file2 = match File::open("./src/lib.rs") {
          Ok(f) => f,
          Err(e) => panic!("{}", e),
        };
        let files = format!("failed to diff {},{},{}",self.counter,i,j).to_string();
        let s = Command::new("diff")
          .arg(v[i].clone())
          .arg(v[j].clone())
          .output()
          .expect(&files)
          .stdout;
        let mut file = File::create(
            format!("diff.{}.{}.{}",self.counter,i,j)).unwrap();
        let _ = file.write(b"1st file:");
        let _ = file.write(v[i].as_bytes());
        let _ = file.write(b"\n");
        let _ = file.write(b"2nd file:");
        let _ = file.write(v[j].as_bytes());
        let _ = file.write(b"\n");
        let _ = file.write_all(&s);
      }
      println!("{:?}",v);
    }
    Some(())
  }
}
