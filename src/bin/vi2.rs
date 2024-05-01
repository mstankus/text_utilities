use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
use glob::glob;

fn main() {
  let args : Vec<_> = env::args().collect();
  let str = format!("diff-{}-{}-*",args[1],args[2]).to_string();
  if let Some(Ok(path)) = glob(&str).expect("hi").next() {
    let f = File::open(path.clone());
    let mut b = BufReader::new(f.unwrap());
    let mut s1 = String::new();
    let mut s2 = String::new();
    let _ = b.read_line(&mut s1);
    let _ = b.read_line(&mut s2);
    println!("vimdiff ../\"{}\" ../\"{}\"",
             &s1[9..].trim(),
             &s2[9..].trim());
  }
}

