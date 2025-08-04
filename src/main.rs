use std::io::{self, Error, Write};
use rust_crud_cli::{Todo, TodoList};

fn take_user_input() -> String {
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error in reading user input!");
    user_input.trim().to_string()
}

fn main() {
    let mut todo_list: TodoList = TodoList::new();
    loop {
        print!(
            "Todo App!\nOperations\n1.Add Todo\n2.Get Todo\n3.Update Todo\n4.Remove Todo\n5.Return All Todos\n6.Return Completed Todos\n7.Return Incomplete Todos\nChoose Operation: "
        );
        let user_choice: u32 = take_user_input()
            .parse()
            .expect("Error in parsing user_choice!");

        match user_choice {
            1 => {
                print!("Enter todo id: ");
                let todo_id: u32 = take_user_input()
                    .parse()
                    .expect("Error in parsing todo_id!");
                print!("Enter todo title: ");
                let todo_title = take_user_input();
                print!("Enter todo description: ");
                let todo_description = take_user_input();
                print!("Enter completion status(true/false): ");
                let completion_status: bool = take_user_input()
                    .parse()
                    .expect("Error in parsing completion status!");

                match todo_list.add_todo(Todo::new(
                    todo_id,
                    todo_title,
                    todo_description,
                    completion_status,
                )) {
                    Ok(()) => {
                        println!("Added todo!");
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
            2 => {
                print!("Enter todo id: ");
                let todo_id: u32 = take_user_input()
                    .parse()
                    .expect("Error in parsing the todo_id");
                match todo_list.get_todo(todo_id) {
                    Ok(todo) => {
                        println!("Todo is: {todo:#?}");
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
            3 => {
                print!("Enter todo id: ");
                let todo_id: u32 = take_user_input()
                    .parse()
                    .expect("Error in parsing the todo_id");
                print!("Enter new todo title: ");
                let new_title = take_user_input();
                print!("Enter new todo description: ");
                let new_description = take_user_input();
                print!("Enter completion status(true/false): ");
                let completion_status: bool = take_user_input()
                    .parse()
                    .expect("Error converting completion status!");
                let updated_todo =
                    Todo::new(todo_id, new_title, new_description, completion_status);

                match todo_list.update_todo(todo_id, updated_todo) {
                    Ok(()) => {
                        println!("Updated todo!");
                    }
                    Err(e) => {
                        println!("e");
                    }
                }
            }
            4 => {
                print!("Enter todo id: ");
                let todo_id: u32 = take_user_input()
                    .parse()
                    .expect("Error in converting todo id!");
                match todo_list.remove_todo(todo_id) {
                    Ok(()) => {
                        println!("Removed todo!");
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
            5 => {
                let list_of_todos = todo_list.return_all_todos();
                println!("All todos: {list_of_todos:#?}");
            }
            6 => {
                let completed_todos = todo_list.return_completed_todos();
                println!("Completed todos: {completed_todos:#?}");
            }
            7 => {
                let incomplete_todos = todo_list.return_incomplete_todos();
                println!("Incomplete todos: {incomplete_todos:#?}");
            }
            _ => {
                println!("Invalid Operation Choosed!");
            }
        }
    }
}
