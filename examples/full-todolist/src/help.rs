//! This module provides help information for the `todolist` command line program

use mingling::macros::{help, r_println};

use crate::{EntryAdd, EntryClean, EntryComplete, EntryList, ErrorDispatcherNotFound};

#[help]
pub fn help_global(_p: ErrorDispatcherNotFound) {
    r_println!(
        "{}",
        r"
Usage: todolist [command] [args]

Commands:
  add            -- Add a new task
  list           -- List all tasks
  complete       -- Mark a task as complete
  clean          -- Clean up completed tasks

Args:
  -h, --help     -- Show this help message
  -V, --version  -- Show the version
  -A, --all      -- All tasks (Clean all / List all)
            "
        .trim()
    );
}

#[help]
pub fn help_add(_p: EntryAdd) {
    r_println!(
        "{}",
        r"
Usage: todolist add [task description]

Add a new task to the todo list.

Example:
  todolist add 'Buy groceries'
  todolist add 'Finish Rust project'
            "
        .trim()
    );
}

#[help]
pub fn help_list(_p: EntryList) {
    r_println!(
        "{}",
        r"
Usage: todolist list

List all tasks.

Example:
  todolist list
            "
        .trim()
    );
}

#[help]
pub fn help_complete(_p: EntryComplete) {
    r_println!(
        "{}",
        r"
Usage: todolist complete [task_id]

Mark a task as complete by its ID.

Example:
  todolist complete 1
  todolist complete 3
            "
        .trim()
    );
}

#[help]
pub fn help_clean(_p: EntryClean) {
    r_println!(
        "{}",
        r"
Usage: todolist clean

Remove all completed tasks from the list.

Example:
  todolist clean
            "
        .trim()
    );
}
