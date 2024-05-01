//use crate::csfunctions::*;
use crate::csextfunctions::*;

pub enum PassYamlStatus {
  NoLinesEncountered,
  WithinYaml,
  OutsideYaml
}

pub struct PassYaml {
  status : PassYamlStatus,
}

impl PassYaml {
  pub fn new() -> PassYaml {
    let status = PassYamlStatus::NoLinesEncountered;
    PassYaml { status }
  }
}

impl Default for PassYaml {
  fn default() -> Self { 
    Self::new()
  }
}

impl CSCExtFunction<String,String> for PassYaml {
  fn invoke(&mut self,t : String) -> Option<String> {
    match self.status {
      PassYamlStatus::NoLinesEncountered => {
        if t=="---" {
          self.status = PassYamlStatus::WithinYaml;
          None
        } else {
          self.status = PassYamlStatus::OutsideYaml;
          Some(t)
        }
      },
      PassYamlStatus::WithinYaml => {
        if t=="---" {
          self.status = PassYamlStatus::OutsideYaml;
        }
        None
      },
      PassYamlStatus::OutsideYaml => {
        Some(t)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pass_yaml1() {
    let mut y = PassYaml::new();
    assert_eq!(y.invoke("---".to_string()),None);
    assert_eq!(y.invoke("abc".to_string()),None);
    assert_eq!(y.invoke("---".to_string()),None);
    assert_eq!(y.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(y.invoke("ghi".to_string()),Some("ghi".to_string()));
  }

  #[test]
  fn test_pass_yaml2() {
    let mut y = PassYaml::new();
    assert_eq!(y.invoke("def".to_string()),Some("def".to_string()));
    assert_eq!(y.invoke("ghi".to_string()),Some("ghi".to_string()));
    assert_eq!(y.invoke("---".to_string()),Some("---".to_string()));
  }
}
