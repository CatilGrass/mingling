use mingling::{REPL, hook::ProgramHook, prelude::*, this};
use std::{env::current_dir, path::PathBuf};

// Resource to store the current directory
#[derive(Clone)]
struct CurrentDir {
    dir: PathBuf,
}

impl Default for CurrentDir {
    fn default() -> Self {
        Self {
            dir: current_dir().unwrap(),
        }
    }
}

fn main() {
    let mut program = ThisProgram::new();

    // Add resource
    program.with_resource(CurrentDir::default());

    // Add dispatchers
    program.with_dispatcher(ChangeDirectoryCommand);
    program.with_dispatcher(ListCommand);
    program.with_dispatcher(ExitCommand);
    program.with_dispatcher(ClearCommand);

    // Add hooks to handle REPL-related events
    program.with_hook(
        ProgramHook::empty()
            .on_repl_begin(|| {
                // Print welcome message
                println!("Welcome!")
            })
            .on_repl_pre_readline(|| {
                // Print prompt
                let res = this::<ThisProgram>().res::<CurrentDir>().unwrap();
                let dir_str: String = res.dir.to_string_lossy().into();
                let prompt = format!(
                    "{}> ",
                    dir_str
                        .replace(&['/', '\\'][..], ">")
                        .trim_start_matches('>')
                        .trim_end_matches('>')
                );
                print!("{}", prompt)
            })
            .on_repl_receive_result(|r| {
                // Print output
                if !r.is_empty() {
                    println!("{}", r.trim())
                }
            }),
    );

    // Start the REPL loop
    program.exec_repl();
}

// Create error route
pack!(ErrorDirectoryNotExist = PathBuf);

// Create commands: cd ls exit
dispatcher!("cd", ChangeDirectoryCommand => ChangeDirectoryEntry);
dispatcher!("ls", ListCommand => ListEntry);
dispatcher!("exit", ExitCommand => ExitEntry);
dispatcher!("clear", ClearCommand => ClearEntry);

// Define data needed for the cd command's execution phase
pack!(StateChangeDirectory = String);

// Define data needed for the ls command's rendering phase
pack!(ResultList = Vec<String>);

// Parse cd command arguments
#[chain]
fn parse_cd_args(prev: ChangeDirectoryEntry) -> Next {
    let join = prev.pick(()).unpack();
    StateChangeDirectory::new(join)
}

// Execute directory change
#[chain]
fn handle_cd(prev: StateChangeDirectory, current_dir: &mut CurrentDir) -> Next {
    let join = prev.inner;
    let new_dir = just_fmt::fmt_path::fmt_path(current_dir.dir.join(join)).unwrap_or_default();

    // If the path is not found, route to error handling
    if !new_dir.exists() {
        return ErrorDirectoryNotExist::new(new_dir).to_render();
    }

    current_dir.dir = new_dir;
    empty_result!()
}

// Get directory contents via the CurrentDir resource
#[chain]
fn handle_ls(_prev: ListEntry, current_dir: &CurrentDir) -> Next {
    let dir = &current_dir.dir;
    let entries: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flat_map(|rd| rd.filter_map(|e| e.ok()))
        .map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                format!("{}/", name)
            } else {
                name
            }
        })
        .collect();

    // Render ResultList
    ResultList::new(entries).to_render()
}

// Render ResultList data
#[renderer]
fn render_list(list: ResultList) {
    for item in list.inner {
        r_println!("{}", item)
    }
}

// Handle exit command event
#[chain]
fn handle_exit(
    _prev: ExitEntry,
    repl: &mut REPL, // Import REPL resource, registered in `exec_repl`, usable directly
) {
    // Set the REPL exit flag; REPL will exit after this loop iteration
    repl.exit = true;
}

// Handle clear command event
#[chain]
fn handle_clear(_prev: ClearEntry) {
    // Clear the terminal screen
    print!("\x1B[2J\x1B[1;1H");
}

// Handle path not found event
#[renderer]
fn render_error_directory_not_exist(err: ErrorDirectoryNotExist) {
    r_println!("Directory not found: {}", err.inner.display())
}

// Handle dispatcher not found event
#[renderer]
fn dispatcher_not_found(prev: DispatcherNotFound) {
    r_println!("Command not found: \"{}\"", prev.join(", "))
}

gen_program!();
