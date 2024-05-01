use std::process::Command;

pub fn to_cygpath(s : &str) -> String {
  let path = Command::new("cygpath")
               .arg("-w")
               .arg(s.trim())
               .output()
               .unwrap()
               .stdout;
  String::from_utf8(path).unwrap()
}
