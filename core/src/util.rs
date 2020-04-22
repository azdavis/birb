use std::fmt;

pub struct SliceDisplay<'a, T> {
  left: &'static str,
  content: &'a [T],
  right: &'static str,
}

impl<'a, T> SliceDisplay<'a, T> {
  pub fn new(left: &'static str, content: &'a [T], right: &'static str) -> Self {
    Self {
      left,
      content,
      right,
    }
  }
}

impl<'a, T> fmt::Display for SliceDisplay<'a, T>
where
  T: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.left)?;
    let mut iter = self.content.iter();
    if let Some(x) = iter.next() {
      write!(f, "{}", x)?;
    }
    for x in iter {
      write!(f, ", {}", x)?;
    }
    write!(f, "{}", self.right)
  }
}
