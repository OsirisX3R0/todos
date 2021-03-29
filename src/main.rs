// Local modules
mod todo;
mod todo_list;
mod todo_status;

fn main() {
    let mut todos: todo_list::TodoList = todo_list::TodoList::new();
    todos.add("Take out the trash");
    todos.add("Cook the dinner");
    todos.add("Wash the dishes");
    todos.add("Go to bed");
    let index: usize = 1;
    print!("Initial Todos: {}", todos);
    print!("\n\nTodo at index {}: {}", index, todos.get(index));
    todos.complete(0);
    todos.complete(1);
    todos.incomplete(1);
    todos.delete(3);
    todos.update_text(1, "Grill the burgers");
    let todo: &todo::Todo = todos.get(0);
    print!("\n\nUpdated Todos: {}", todos);
    print!(
        "\n\nNew todo at index {}: '{}' => {}",
        index,
        todo.text(),
        if todo.is_completed() {
            "completed"
        } else {
            "incomplete"
        }
    );
    todos.clear();
    print!("\n\nCleared todos: {}", todos)
}
