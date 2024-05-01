use std::io::*;
use std::fs::File;

pub enum CompareFiles {
  FirstShorter(usize),
  SecondShorter(usize),
  SameUntil(usize),
  Same,
  Error
}

pub fn compare_files(read1 : &mut BufReader<File>, read2 : &mut BufReader<File>) -> CompareFiles {
  let mut cmp = 0;
  loop {
    let buffer1 = match read1.fill_buf() {
      Ok(buf) => buf,
      Err(_x) => { return CompareFiles::Error;}
    };
    let buffer2 = match read2.fill_buf() {
      Ok(buf) => buf,
      Err(_x) => { return CompareFiles::Error;}
    };
    let len1 = buffer1.len();
    let len2 = buffer2.len();
    if len1==0 { 
      if len2==0 {
        return CompareFiles::Same
      } else {
        return CompareFiles::FirstShorter(cmp)
      }
    } else if len2==0 {
      return CompareFiles::SecondShorter(cmp)
    }
    let mut i = 0;
    let min = std::cmp::min(len1,len2);
    while buffer1[i]==buffer2[i] && i<min {
      cmp += 1;
      i += 1;
    }
    if i<min {
      return CompareFiles::SameUntil(cmp+i)
    } else {
      read1.consume(min);
      read2.consume(min);
      cmp += min;
    }
  }
}
