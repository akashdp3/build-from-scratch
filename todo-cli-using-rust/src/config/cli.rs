use clap::Parser;
use std::io::{self, Write};

use crate::commands::todo::TodoService;
use crate::commands::{Command, Todo};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run() -> Self {
        Self::parse()
    }

    pub fn execute(&self) {
        let mut todo = Todo::new();
        let mut stdout = io::stdout();
        self.execute_with(&mut todo, &mut stdout);
    }

    pub fn execute_with<T: TodoService, W: Write>(&self, todo: &mut T, writer: &mut W) {
        match &self.command {
            Command::Add { title } => match todo.add(title.as_str()) {
                Ok(_) => {
                    let _ = writeln!(writer, "Task added successfully!");
                }
                Err(e) => {
                    let _ = writeln!(writer, "Error: {}", e);
                }
            },
            Command::List => match todo.list() {
                Ok(tasks) => {
                    let _ = writeln!(writer, "{}", tasks.join("\n"));
                }
                Err(e) => {
                    let _ = writeln!(writer, "Error: {}", e);
                }
            },
            Command::Done { task_id } => match todo.change_status(*task_id, "done") {
                Ok(_) => {
                    let _ = writeln!(writer, "Task marked as done successfully!");
                }
                Err(e) => {
                    let _ = writeln!(writer, "Error: {}", e);
                }
            },
            Command::Delete { task_id } => match todo.delete(*task_id) {
                Ok(_) => {
                    let _ = writeln!(writer, "Task deleted successfully!");
                }
                Err(e) => {
                    let _ = writeln!(writer, "Error: {}", e);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct FakeTodo {
        add_result: Result<(), &'static str>,
        list_result: Result<Vec<String>, &'static str>,
        change_status_result: Result<(), &'static str>,
        delete_result: Result<(), &'static str>,
        add_calls: RefCell<Vec<String>>,
        list_calls: RefCell<u32>,
        change_status_calls: RefCell<Vec<(u8, String)>>,
        delete_calls: RefCell<Vec<u8>>,
    }

    impl FakeTodo {
        fn with_defaults() -> Self {
            Self {
                add_result: Ok(()),
                list_result: Ok(vec![]),
                change_status_result: Ok(()),
                delete_result: Ok(()),
                add_calls: RefCell::new(Vec::new()),
                list_calls: RefCell::new(0),
                change_status_calls: RefCell::new(Vec::new()),
                delete_calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl TodoService for FakeTodo {
        fn add(&mut self, title: &str) -> Result<(), &str> {
            self.add_calls.borrow_mut().push(title.to_string());
            self.add_result.clone()
        }

        fn list(&self) -> Result<Vec<String>, &str> {
            *self.list_calls.borrow_mut() += 1;
            self.list_result.clone()
        }

        fn change_status(&mut self, task_id: u8, task_status: &str) -> Result<(), &str> {
            self.change_status_calls
                .borrow_mut()
                .push((task_id, task_status.to_string()));
            self.change_status_result.clone()
        }

        fn delete(&mut self, task_id: u8) -> Result<(), &str> {
            self.delete_calls.borrow_mut().push(task_id);
            self.delete_result.clone()
        }
    }

    #[test]
    fn execute_add_success_prints_message() {
        let cli = Cli {
            command: Command::Add {
                title: "Task".into(),
            },
        };
        let mut todo = FakeTodo::with_defaults();
        let mut output = Vec::new();

        cli.execute_with(&mut todo, &mut output);

        assert_eq!(todo.add_calls.borrow().as_slice(), &["Task".to_string()]);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Task added successfully!\n"
        );
    }

    #[test]
    fn execute_add_failure_prints_error() {
        let cli = Cli {
            command: Command::Add {
                title: "Task".into(),
            },
        };
        let mut todo = FakeTodo {
            add_result: Err("nope"),
            ..FakeTodo::with_defaults()
        };
        let mut output = Vec::new();

        cli.execute_with(&mut todo, &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "Error: nope\n");
    }

    #[test]
    fn execute_list_success_prints_tasks() {
        let cli = Cli {
            command: Command::List,
        };
        let mut todo = FakeTodo {
            list_result: Ok(vec!["1. First".into(), "2. Second".into()]),
            ..FakeTodo::with_defaults()
        };
        let mut output = Vec::new();

        cli.execute_with(&mut todo, &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "1. First\n2. Second\n");
    }

    #[test]
    fn execute_done_failure_prints_error() {
        let cli = Cli {
            command: Command::Done { task_id: 2 },
        };
        let mut todo = FakeTodo {
            change_status_result: Err("missing"),
            ..FakeTodo::with_defaults()
        };
        let mut output = Vec::new();

        cli.execute_with(&mut todo, &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "Error: missing\n");
        assert_eq!(
            todo.change_status_calls.borrow().as_slice(),
            &[(2, "done".to_string())]
        );
    }

    #[test]
    fn execute_delete_success_prints_message() {
        let cli = Cli {
            command: Command::Delete { task_id: 3 },
        };
        let mut todo = FakeTodo::with_defaults();
        let mut output = Vec::new();

        cli.execute_with(&mut todo, &mut output);

        assert_eq!(todo.delete_calls.borrow().as_slice(), &[3]);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Task deleted successfully!\n"
        );
    }
}
