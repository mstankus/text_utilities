
pub trait CSFunction<In,Out> {
  fn invoke(&self,x : In) -> Out;
}

pub trait CSSelfMap<In> {
  fn invoke(&self,x : In) -> In;
}

