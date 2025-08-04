# 📝 Rust CLI Todo App

A simple command-line Todo application written in Rust. This project allows users to add, retrieve, update, delete, and view their todos directly from the terminal. It demonstrates basic use of Rust's `HashMap`, error handling, and user input handling.

## 🚀 Features

- Add new todos with an ID, title, description, and completion status
- Retrieve a todo by its ID
- Update existing todos
- Delete todos
- View:
  - All todos
  - Completed todos
  - Incomplete todos

## 📁 Project Structure

```

.
├── src
│   ├── lib.rs     # Library file containing the `Todo` and `TodoList` logic
│   └── main.rs    # CLI logic and user interaction
├── Cargo.toml     # Project manifest

````

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60+ recommended)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/<your-username>/rust-cli-todo.git
cd rust-cli-todo

# Run the project
cargo run
````

## 📦 Example Usage

```bash
Todo App!
Operations
1.Add Todo
2.Get Todo
3.Update Todo
4.Remove Todo
5.Return All Todos
6.Return Completed Todos
7.Return Incomplete Todos
Choose Operation: 1
Enter todo id: 1
Enter todo title: Buy groceries
Enter todo description: Milk, eggs, and bread
Enter completion status(true/false): false
Added todo!
```

## 🔧 Tech Stack

* Rust
* Standard Library (`std::collections::HashMap`, `std::io`)

## 🧪 Possible Improvements

* Persistent storage (e.g., JSON or SQLite)
* Tagging and due dates
* Sorting and filtering
* Command-line argument support using `clap` or `structopt`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Made with ❤️ in Rust.