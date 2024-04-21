use crate::csfunctions::*;

pub struct ComposeCSSelfMap<In : Sized> {
  data: Vec<Box<dyn CSSelfMap::<In>>>
}

impl<In> ComposeCSSelfMap<In> {
  pub fn new(data : Vec<Box<dyn CSSelfMap::<In>>>) -> ComposeCSSelfMap<In> {
    ComposeCSSelfMap::<In> { data }
  }
}

impl<In> CSSelfMap<In> for ComposeCSSelfMap<In> {
  fn invoke(&self,mut x : In) -> In {
    for item in &self.data {
      x = item.invoke(x);
    }
    x
  }
}
