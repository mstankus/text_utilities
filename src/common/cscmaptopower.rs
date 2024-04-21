
pub trait CSCMapToPower<In> {
  fn invoke(&mut self,x : In) -> Vec::<In>;
}
