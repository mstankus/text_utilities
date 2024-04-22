use std::cmp::Ordering;

pub enum ZeroToInfinityEnum {
  Value(i64),
  Infinity,
}

pub struct ZeroToInfinity {
  e : ZeroToInfinityEnum,
}

impl ZeroToInfinity {
  pub fn compare(&self,e: &ZeroToInfinity)  -> Ordering {
    match self.e {
      ZeroToInfinityEnum::Infinity => {
        match e.e {
          ZeroToInfinityEnum::Infinity => {
            Ordering::Equal
          },
          ZeroToInfinityEnum::Value(_) => {
            Ordering::Greater
          }
        }
      },
      ZeroToInfinityEnum::Value(x) => {
        match e.e {
          ZeroToInfinityEnum::Infinity => {
            Ordering::Less

          },
          ZeroToInfinityEnum::Value(y) => {
            x.cmp(&y)
          }
        }
      }
    }
  }
}
