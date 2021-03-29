#[derive(Debug)]
pub enum TodoStatus {
  Complete,
  Incomplete,
}

impl PartialEq for TodoStatus {
  fn eq(&self, other: &Self) -> bool {
    match self {
      // If status is complete...
      TodoStatus::Complete => match other {
        // Ensure the other status is complete as well
        TodoStatus::Complete => true,
        _ => false,
      },
      // If status is incomplete...
      TodoStatus::Incomplete => match other {
        // Ensure the other status is incomplete as well
        TodoStatus::Incomplete => true,
        _ => false,
      },
    }
  }
}

#[cfg(test)]
// mod tests {
//   use super::*;
#[test]
fn should_be_equal() {
  assert_eq!(TodoStatus::Complete == TodoStatus::Complete, true)
}

#[test]
fn should_not_be_equal() {
  assert_ne!(TodoStatus::Complete == TodoStatus::Incomplete, true)
}
// }
