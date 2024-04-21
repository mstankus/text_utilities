use crate::cscfunctions::*;

pub struct ComposeCSCSelfMap<In : Sized> {
  data: Vec<Box<dyn CSCSelfMap::<In>>>
}

impl<In> ComposeCSCSelfMap<In> {
  pub fn new(data : Vec<Box<dyn CSCSelfMap::<In>>>) -> ComposeCSCSelfMap<In> {
    ComposeCSCSelfMap::<In> { data }
  }
}

impl<In> CSCSelfMap<In> for ComposeCSCSelfMap<In> {
  fn invoke(&mut self,mut x : In) -> In {
    for item in &mut self.data {
      x = item.invoke(x);
    }
    x
  }
}
