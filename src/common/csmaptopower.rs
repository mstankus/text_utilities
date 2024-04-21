
pub trait CSMapToPower<In> {
  fn invoke(&self,x : In) -> Vec::<In>;
}
