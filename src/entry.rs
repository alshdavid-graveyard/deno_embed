use std::path::PathBuf;

pub enum Entry {
  File(PathBuf),
  Eval(String),
}
