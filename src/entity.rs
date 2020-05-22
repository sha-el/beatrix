pub trait Entity {
  fn table_name(&self) -> String;
  // fn create(&self) -> &str;
  // fn select(&self) -> &str;
  // fn pk(&self) -> &str;
}