use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

pub fn count_lines(s : &str,
                   mut h : HashMap::<String,i64>,
                   val : i64) -> 
                   HashMap::<String,i64> {
  match h.entry(s.to_string()) {
    Occupied(mut e) => { *e.get_mut() += val; }
    Vacant(e) => { e.insert(val); }
  }
  h
}

pub fn remove_zero_values(mut h: HashMap::<String,i64>) -> HashMap::<String,i64> {
  h.retain(|_k,v| *v!=0);
  h
}

#[derive(PartialEq,Debug)]
pub enum SummarizedValues {
  Positive(i64),
  Negative(i64),
  Mixed(i64,i64,i64),// neg,zero,positive
  Empty
}

pub fn summarize<K>(h: HashMap<K,i64>) -> SummarizedValues {
  if h.is_empty() { return SummarizedValues::Empty }
  let mut p : i64 = 0;
  let mut n : i64 = 0;
  let mut z : i64 = 0;
  for (_,value) in h.iter() {
    match value.cmp(&0) {
      Ordering::Equal => { z += 1}, 
      Ordering::Greater => { p += value}, 
      Ordering::Less => { n += -value}, 
    }
  }
  if z==0 {
    if p>0 {
      if n>0 {
        SummarizedValues::Mixed(n,z,p)
      } else {
        SummarizedValues::Positive(p)
      }
    } else if n>0 { 
      SummarizedValues::Negative(n)
    } else {
      SummarizedValues::Mixed(0,0,0)
    }
  } else {
    SummarizedValues::Mixed(n,z,p)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn diff_count_lines_1() {
    let mut h = HashMap::new();
    h = count_lines("a",h,2);
    h = count_lines("b",h,3);
    h = count_lines("a",h,-2);
    h = count_lines("b",h,-3);
    h = remove_zero_values(h);
    println!("{:?}",h);
    assert_eq!(summarize(h),SummarizedValues::Empty);
  }
  #[test]
  fn diff_count_lines_2() {
    let mut h = HashMap::new();
    h = count_lines("a",h,3);
    h = count_lines("b",h,3);
    h = count_lines("a",h,-2);
    h = count_lines("b",h,-3);
    h = remove_zero_values(h);
    println!("{:?}",h);
    assert_eq!(summarize(h),SummarizedValues::Positive(1));
  }

  #[test]
  fn diff_count_lines_3() {
    let mut h = HashMap::new();
    h = count_lines("a",h,2);
    h = count_lines("b",h,4);
    h = count_lines("a",h,-2);
    h = count_lines("b",h,-25);
    h = remove_zero_values(h);
    println!("{:?}",h);
    assert_eq!(summarize(h),SummarizedValues::Negative(21));
  }
}
