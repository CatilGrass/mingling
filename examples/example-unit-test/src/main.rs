//! Example Unit Test
//!
//! > This example shows how to write unit tests for Chain and Renderer in Mingling
//!
//! ```bash
//! cargo test --manifest-path examples/example-unit-test/Cargo.toml
//! ```

use mingling::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mingling::macros::entry;
    use mingling::{assert_member_id, assert_render_result};

    // --------- IMPORTANT ---------
    #[test]
    fn test_handle_hello() {
        let hello_without_args = handle_hello(entry!()).into();
        assert_render_result!(hello_without_args);
        assert_member_id!(hello_without_args, ThisProgram::ErrorNoNameProvided);

        let hello_with_registered_name = handle_hello(entry!("Alice")).into();
        assert_render_result!(hello_with_registered_name);
        assert_member_id!(
            hello_with_registered_name,
            ThisProgram::ErrorNameNotAvailable
        );

        let hello_with_long_name = handle_hello(entry!("It's a VeryLongName")).into();
        assert_render_result!(hello_with_long_name);
        assert_member_id!(hello_with_long_name, ThisProgram::ErrorNameTooLong);

        let hello_with_valid_name = handle_hello(entry!("Peter")).into();
        assert_render_result!(hello_with_valid_name);
    }

    #[test]
    fn test_render_result_name() {
        let r = render_result_name(ResultName::new("Peter".into()));
        assert_eq!(r, "Hello, Peter!\n")
    }

    #[test]
    fn test_render_error_no_name_provided() {
        let r = render_error_no_name_provided(ErrorNoNameProvided::default());
        assert_eq!(r, "No name provided\n")
    }

    #[test]
    fn test_render_error_name_not_available() {
        let r = render_error_name_not_available(ErrorNameNotAvailable::default());
        assert_eq!(r, "Name not available\n")
    }

    #[test]
    fn test_render_error_name_too_long() {
        let r = render_error_name_too_long(ErrorNameTooLong::new(17));
        assert_eq!(r, "Name too long: 17 > 10\n")
    }
    // --------- IMPORTANT ---------
}

dispatcher!("hello", CMDHello => EntryHello);

pack!(ErrorNoNameProvided = ());
pack!(ErrorNameTooLong = u16);
pack!(ErrorNameNotAvailable = ());

pack!(ResultName = String);

static VEC_REGISTERED_NAMES: &[&str] = &["Alice", "Bob", "Charlie", "David", "Eve"];

#[chain]
fn handle_hello(args: EntryHello) -> Next {
    let Some(name) = args.inner.first().cloned() else {
        return ErrorNoNameProvided::default().to_render();
    };

    if name.len() > 10 {
        return ErrorNameTooLong::new(name.len() as u16).to_render();
    }

    if VEC_REGISTERED_NAMES.contains(&name.as_str()) {
        return ErrorNameNotAvailable::default().to_render();
    }

    ResultName::new(name).to_render()
}

#[renderer]
fn render_result_name(name: ResultName) -> String {
    r_println!("Hello, {}!", *name);
}

#[renderer]
fn render_error_no_name_provided(_: ErrorNoNameProvided) -> String {
    r_println!("No name provided");
}

#[renderer]
fn render_error_name_not_available(_: ErrorNameNotAvailable) -> String {
    r_println!("Name not available");
}

#[renderer]
fn render_error_name_too_long(len: ErrorNameTooLong) -> String {
    r_println!("Name too long: {} > 10", *len);
}

#[renderer]
fn render_dispatcher_not_found(err: DispatcherNotFound) {
    r_println!("Command not found: \"{}\"", err.inner.join(" "));
}

gen_program!();

fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDHello);
    program.exec_and_exit();
}
