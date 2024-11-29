

// src/main.rs
use std::io::{self, Write};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}




impl TodoList {
    
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add_todo(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
    }

    fn remove_todo(&mut self, id: usize) -> bool {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(pos);
            true
        } else {
            false
        }
    }

    fn toggle_todo(&mut self, id: usize) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
            true
        } else {
            false
        }
    }

    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string(&self)?;
        fs::write("todos.json", json)
    }

    fn load_from_file() -> io::Result<Self> {
        match fs::read_to_string("todos.json") {
            Ok(contents) => {
                let todo_list: TodoList = serde_json::from_str(&contents)?;
                Ok(todo_list)
            }
            Err(_) => Ok(TodoList::new()),
        }
    }
}

fn main() {
    let mut todo_list = TodoList::load_from_file().unwrap_or_else(|_| TodoList::new());

    loop {
        println!("\n=== Todo List ===");
        println!("1. Add todo");
        println!("2. List todos");
        println!("3. Toggle todo");
        println!("4. Remove todo");
        println!("5. Save and exit");

        print!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(1) => {
                print!("Enter todo title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add_todo(title.trim().to_string());
                println!("Todo added!");
            }
            Ok(2) => {
                println!("\nCurrent todos:");
                for todo in &todo_list.todos {
                    println!("[{}] {}. {}", 
                        if todo.completed { "x" } else { " " },
                        todo.id,
                        todo.title
                    );
                }
            }
            Ok(3) => {
                print!("Enter todo ID to toggle: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse() {
                    if todo_list.toggle_todo(id) {
                        println!("Todo toggled!");
                    } else {
                        println!("Todo not found!");
                    }
                }
            }
            Ok(4) => {
                print!("Enter todo ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse() {
                    if todo_list.remove_todo(id) {
                        println!("Todo removed!");
                    } else {
                        println!("Todo not found!");
                    }
                }
            }
            Ok(5) => {
                todo_list.save_to_file().expect("Failed to save todos");
                println!("Todos saved! Goodbye!");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut list = TodoList::new();
        list.add_todo("Test todo".to_string());
        assert_eq!(list.todos.len(), 1);
        assert_eq!(list.todos[0].title, "Test todo");
    }

    #[test]
    fn test_toggle_todo() {
        let mut list = TodoList::new();
        list.add_todo("Test todo".to_string());
        assert!(!list.todos[0].completed);
        list.toggle_todo(1);
        assert!(list.todos[0].completed);
    }
}