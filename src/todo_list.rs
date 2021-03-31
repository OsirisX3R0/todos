use std::fmt::{Display, Formatter, Result};
use std::vec::Vec;

use crate::todo;

#[derive(Debug)]
/// A list of `Todos`
pub struct TodoList {
  /// The list of todos
  todos: Todos,
}

impl PartialEq for TodoList {
  fn eq(&self, other: &Self) -> bool {
    // Start with equal as true
    let mut equal = true;
    let todo1_len = self.todos.0.len();
    let todo2_len = other.todos.0.len();

    // If the lengths do not match...
    if todo1_len != todo2_len {
      equal = false
    } else {
      // If at least one length is zero
      if todo1_len != 0 || todo2_len != 0 {
        // Iterate/enumerate over todos
        for (i, _x) in self.todos.0.iter().enumerate() {
          // If last value was equal...
          if equal {
            // Check equality and set equal variable
            let todo1 = &self.todos.0[i];
            let todo2 = &other.todos.0[i];
            equal = todo1.eq(todo2);
          }
        }
      }
    }

    equal
  }
}

impl Display for TodoList {
  fn fmt(&self, f: &mut Formatter) -> Result {
    self.todos.0.iter().fold(Ok(()), |result, todo| {
      result.and_then(|_| writeln!(f, "{}", todo))
    })
  }
}

// For listing todos; We can avoid using this with the above implementation
// MORE TROUBLE THAN IT'S WORTH!!!!!!!!
#[derive(Debug)]
pub struct Todos(pub Vec<todo::Todo>);

impl Display for Todos {
  fn fmt(&self, f: &mut Formatter) -> Result {
    self.0.iter().fold(Ok(()), |result, todo| {
      result.and_then(|_| writeln!(f, "{}", todo))
    })
  }
}

impl TodoList {
  /// Creates a new TodoList
  pub fn new() -> TodoList {
    TodoList {
      todos: Todos(Vec::new()),
    }
  }

  /// Lists the todos in the TodoList
  // pub fn list(&self) -> &Vec<todo::Todo> {
  //   &self.todos
  // }

  /// Get a single todo::Todo by index
  pub fn get(&self, index: usize) -> &todo::Todo {
    &self.todos.0[index]
  }

  /// Add a todo to the list
  pub fn add(&mut self, text: &str) {
    let todo: todo::Todo = todo::Todo::new(text);
    self.todos.0.push(todo);
  }

  /// Update the text of a todo by index
  pub fn update_text(&mut self, index: usize, text: &str) {
    let todo: &mut todo::Todo = &mut self.todos.0[index];
    todo.update_text(text)
  }

  /// Mark a todo as complete by index
  pub fn complete(&mut self, index: usize) {
    let todo: &mut todo::Todo = &mut self.todos.0[index];
    todo.complete()
  }

  /// Mark a todo as incomplete by index
  pub fn incomplete(&mut self, index: usize) {
    let todo: &mut todo::Todo = &mut self.todos.0[index];
    todo.incomplete()
  }

  /// Delete a todo by index
  pub fn delete(&mut self, index: usize) {
    self.todos.0.remove(index);
  }

  /// Clear all todos
  pub fn clear(&mut self) {
    self.todos = Todos(Vec::new())
  }
}

#[cfg(test)]
// mod tests {
//   use super::*;
// use crate::todo_status;
#[test]
fn should_create_new_todo_list() {
  let default_list = TodoList::new();
  let new_list = TodoList {
    todos: Todos(Vec::new()),
  };
  assert_eq!(default_list == new_list, true);
}

#[test]
fn should_not_be_equal() {
  let mut default_list = TodoList::new();
  let mut new_list = TodoList {
    todos: Todos(Vec::new()),
  };
  new_list.add("A todo!");
  assert_ne!(default_list == new_list, true);
  default_list.add("A different todo!");
  assert_ne!(default_list == new_list, true);
}

// }
