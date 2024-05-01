
pub trait GetLine {
  fn get_line(&mut self) -> Option<String>;
}

pub struct GetLineFromFile {
  buf : BufReader<File>
}

impl GetLineFromFile {
  pub fn new(s : &str) -> Self {
    match File::open(s) {
      Ok(f) => {
        Self { buf : BufReader::new(f) }
      },
      Err(_) => { panic!("hi");}
    }
  }
}

impl GetLine for GetLineFromFile {
  fn get_line(&mut self) -> Option<String> {

  }
}

pub trait GetLineFactory {
  fn get_line_factory(&mut self,s: &str) -> impl GetLine;
}

pub struct GetLineFileCreator;

pub impl GetLineFactory for GetLineFileCreator {
  fn get_line_factory(&mut self,s: &str) -> GetLineFromFile {
    GetLineFromFile::new(s)
  }
}
