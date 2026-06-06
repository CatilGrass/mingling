use std::{env::current_dir, process::exit};

use crate::{
    CMDCompletion, ThisProgram,
    display::markdown,
    proj_mgr::{CMDInstall, CMDListNamespace, CMDRemoveNamespace},
    res::ResCurrentDir,
};
use mingling::{
    Program,
    hook::ProgramHook,
    macros::program_setup,
    setup::{ExitCodeSetup, GeneralRendererSetup, HelpFlagSetup, QuietFlagSetup},
};

pub fn run() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    // Preprocess args to handle cargo-mling invocations
    let mut args: Vec<String> = std::env::args().collect();
    if args.first().is_some_and(|a| a.contains("cargo-mling")) {
        args[0] = "cargo-mling".to_string();
    }
    if args.get(1).is_some_and(|a| a == "mling") {
        args.remove(1);
    }

    // Build program with preprocessed args
    let mut program = Program::<ThisProgram>::new_with_args(args);

    // Intercept Version
    program.global_flag(["-V", "--version"], |_| {
        eprintln!(include_str!("helps/version.txt"));
        exit(0)
    });

    // Intercept Help
    program.with_hook(ProgramHook::empty().on_post_dispatch(|c| match c {
        // When dispatcher is not found
        ThisProgram::ErrorDispatcherNotFound
            // And user requests Help
            if ThisProgram::this().user_context.help => {
                // Print help
                eprintln!("{}", markdown(include_str!("helps/mling_help.txt")));
                exit(0)
            }
        _ => {}
    }));

    // Setups
    program.with_setup(HelpFlagSetup::new(["-h", "--help"]));
    program.with_setup(GeneralRendererSetup);
    program.with_setup(StandardOutputSetup);
    program.with_setup(ExitCodeSetup::default());

    // Resources
    program.with_resource(ResCurrentDir {
        path: current_dir().unwrap(),
    });

    // Commands
    program.with_dispatcher(CMDCompletion);
    program.with_dispatcher(CMDInstall);
    program.with_dispatcher(CMDListNamespace);
    program.with_dispatcher(CMDRemoveNamespace);

    // Execute
    let quiet = program.stdout_setting.quiet;
    let error_output = program.stdout_setting.error_output && !quiet;
    let render_output = program.stdout_setting.render_output && !quiet;
    let result = program.exec_without_render().unwrap();
    if !result.is_empty() {
        if result.exit_code == 0 && render_output {
            println!("{}", result.trim());
        } else if error_output {
            eprintln!("{}", result.trim());
        }
    }
    exit(result.exit_code);
}

#[program_setup]
fn standard_output_setup(program: &mut Program<ThisProgram>) {
    program.with_setup(QuietFlagSetup::new("--silence"));
    program.global_flag(["--no-error"], |program| {
        program.stdout_setting.error_output = false;
    });
    program.global_flag(["--no-result"], |program| {
        program.stdout_setting.render_output = false;
    });
    program.global_flag(["--silence", "--quiet"], |program| {
        program.stdout_setting.quiet = true;
    });
}
