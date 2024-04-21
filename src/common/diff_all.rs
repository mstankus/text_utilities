use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn diff_all(vec : Vec::<String>,cnt : usize) {
  for i in 0..(vec.len()) {
    for j in (i+1)..(vec.len()) {
      let diffname = format!("diff.{}.{}.{}",cnt,i,j).to_string();
      let files = format!("failed to {}",diffname).to_string();
      let s = Command::new("diff")
               .arg(vec[i].clone())
               .arg(vec[j].clone())
               .output()
               .expect(&files)
               .stdout;
      let mut file = File::create(diffname).unwrap();
      let _ = file.write(b"1st file:");
      let _ = file.write(vec[i].as_bytes());
      let _ = file.write(b"\n");
      let _ = file.write(b"2nd file:");
      let _ = file.write(vec[j].as_bytes());
      let _ = file.write(b"\n");
      let _ = file.write_all(&s);
    }
  }
}
