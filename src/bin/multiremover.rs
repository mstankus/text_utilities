use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::process::Command;
use glob::glob;

fn main() {
  let args : Vec<_> = env::args().collect();
  let str = format!("diff-{}-*",args[1]).to_string();
  if let Some(Ok(path)) = glob(&str).expect("hi").next() {
    let f = File::open(path.clone());
    let mut b = BufReader::new(f.unwrap());
    let mut s = String::new();
    let _ = b.read_line(&mut s);
    println!("item:{:?}",path);
    println!("rm \"{}\"",&s[9..].trim());
  }
  println!("{:?}",Command::new("rm")
      .arg(format!("diff-{}-*",args[1]).to_string())
      .arg(format!("diff-*-{}-*",args[1]).to_string())
      .output()
      .expect("remove did not work!")
      .stdout);
}

