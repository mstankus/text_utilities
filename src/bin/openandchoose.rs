use std::fs::File;
use std::io::{BufRead,BufReader};
use std::process::Command;

fn main() {
  let f = File::open("hi.txt");
  if f.is_err() { panic!("Cannot open file");}
  let mut buf = BufReader::new(f.unwrap());
  let mut vec : Vec<String> = Vec::new();
  loop {
    let mut s = String::new();
    match buf.read_line(&mut s) {
      Ok(len) => {
        if len==0 { break; }
        let u = s.clone();
        let trim = u.trim();
        if trim.is_empty() {
          println!("{:?}",vec);
          let c = Command::new("open")
              .arg(vec[0].clone())
              .output()
              .expect("Ack!")
              .stdout;
          let s = match String::from_utf8(c) {
             Ok(s) => s,
             Err(_) => panic!("cannot convert output of open to a string")
          };
          println!("s{:?}",s);
          panic!("blank line");
        } else {
          vec.push(trim.to_string());
        }
      },
      Err(_) => {panic!("Ack!");}
    }
  }
}
