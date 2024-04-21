use std::io::BufReader;
use std::fs::File;
use crate::cscfunctions::*;
use crate::csextfunctions::*;
use crate::file_information::*;

pub struct ObtainFromFile;
impl CSCExtFunction::<String,BufReader<File>> for ObtainFromFile {
  fn invoke(&mut self,x : String) -> Option::<BufReader::<File>> {
    get_bufreader(&x)
  }
}

pub struct BufReaderToVec;
impl CSCFunction::<BufReader::<File>,Vec::<String>> for BufReaderToVec {
  fn invoke(&mut self,x : BufReader::<File>) -> Vec::<String> {
    buf_reader_to_vec(x)
  }
}
