use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
use glob::glob;

fn open_some_file(s : &str) -> Option<BufReader<File>> {
  if let Some(Ok(path))=glob(s).expect("hi").next() {
    let f = File::open(path.clone());
    Some(BufReader::new(f.unwrap()))
  } else {
    None
  }
}

fn open_some_file_read_lines(s : &str,n : usize) -> Option<Vec<String>>{
  match open_some_file(s) {
    Some(mut reader) => {
      let mut v = Vec::new(); 
       for _i in 0..n {
         let mut s = String::new();
         let _ = reader.read_line(&mut s);
         v.push(s);
       }
       Some(v)
    },
    None => None
  }
}

fn main() {
  let args : Vec<_> = env::args().collect();
  let str1 = format!("diff-{}-*",args[1]).to_string();
  let str2 = format!("diff-*-{}-*",args[1]).to_string();
  if let Some(lines) = open_some_file_read_lines(&str1,1) {
    println!("rm ../{}" ,&lines[0][9..]);
  } else if let Some(lines) = open_some_file_read_lines(&str2,2) {
    println!("rm ../{}" ,&lines[1][9..]);
  } else {
    panic!("Ack");
  }
  println!("rm diff-{0}-* diff-*-{0}-*",args[1]);
}
