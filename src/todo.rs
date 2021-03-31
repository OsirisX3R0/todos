use std::fmt::{Display, Formatter, Result};

use crate::todo_status;

#[derive(Debug)]
/// A single todo
pub struct Todo {
  /// The text for the todo
  text: String,
  /// The completed status of the todo
  completed: todo_status::TodoStatus,
}

impl PartialEq for Todo {
  fn eq(&self, other: &Self) -> bool {
    self.text == other.text && self.completed == other.completed
  }
}

impl Display for Todo {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "[{}] {}", self.completed, self.text)
  }
}

impl Todo {
  /// Create a new todo
  pub fn new(text: &str) -> Todo {
    Todo {
      text: String::from(text),
      completed: todo_status::TodoStatus::Incomplete,
    }
  }

  /// Return the text of a todo
  pub fn text(&self) -> &String {
    &self.text
  }

  /// Return the completed status of a todo
  pub fn is_completed(&self) -> bool {
    match self.completed {
      todo_status::TodoStatus::Complete => true,
      _ => false,
    }
  }

  /// Update the text of a todo
  pub fn update_text(&mut self, text: &str) {
    self.text = String::from(text);
  }

  /// Mark a todo as complete by index
  pub fn complete(&mut self) {
    self.completed = todo_status::TodoStatus::Complete;
  }

  /// Mark a todo as incomplete by index
  pub fn incomplete(&mut self) {
    self.completed = todo_status::TodoStatus::Incomplete;
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
#[test]
fn should_create_new_todo() {
  let default_todo = Todo::new("New todo");
  let new_todo = Todo {
    text: String::from("New todo"),
    completed: todo_status::TodoStatus::Incomplete,
  };
  assert_eq!(default_todo == new_todo, true);
}

#[test]
fn should_return_text() {
  let todo = Todo::new("New todo");

  assert_eq!(todo.text() == "New todo", true)
}

#[test]
fn should_return_is_completed() {
  let todo = Todo::new("New todo");

  assert_eq!(todo.is_completed(), false)
}

#[test]
fn should_update_text() {
  let mut todo = Todo::new("New todo");
  todo.update_text("New text");

  assert_eq!(todo.text() == "New text", true)
}

#[test]
fn should_start_as_incomplete() {
  let todo = Todo::new("New todo");

  assert_eq!(todo.is_completed(), false)
}

#[test]
fn should_mark_complete() {
  let mut todo = Todo::new("New todo");
  todo.complete();

  assert_eq!(todo.is_completed(), true)
}

#[test]
fn should_mark_incomplete() {
  let mut todo = Todo::new("New todo");
  todo.complete();
  todo.incomplete();

  assert_eq!(todo.is_completed(), false)
}

#[test]
fn should_not_be_equal() {
  let default_todo = Todo::new("New todo");
  let new_todo = Todo {
    text: String::from("Different todo"),
    completed: todo_status::TodoStatus::Incomplete,
  };

  assert_ne!(default_todo == new_todo, true)
}
// }
