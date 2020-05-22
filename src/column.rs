pub trait Column {
  fn name(&self) -> &str;
}

pub struct Varchar;
pub struct Integer;
pub struct Boolean;

impl Column for Varchar {
  fn name(&self) -> &str {
    "varchar"
  }
}

impl Column for Integer {
  fn name(&self) -> &str {
    "integer"
  }
}

impl Column for Boolean {
  fn name(&self) -> &str {
    "boolean"
  }
}