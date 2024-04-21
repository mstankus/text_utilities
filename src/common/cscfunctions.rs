
pub trait CSCFunction<In,Out> {
  fn invoke(&mut self,x : In) -> Out;
}


pub trait CSCSelfMap<In> {
  fn invoke(&mut self,x : In) -> In;
}
