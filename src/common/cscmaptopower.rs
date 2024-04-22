
pub trait CSCMapToPower<In> {
  fn invoke_mut(&mut self,x : In) -> Vec::<In>;
}
