use just_template::tmpl_param;

use crate::ShellFlag;

const TMPL_COMP_BASH: &str = include_str!("../../tmpls/comps/bash.sh");
const TMPL_COMP_ZSH: &str = include_str!("../../tmpls/comps/zsh.zsh");
const TMPL_COMP_FISH: &str = include_str!("../../tmpls/comps/fish.fish");
const TMPL_COMP_PWSH: &str = include_str!("../../tmpls/comps/pwsh.ps1");

/// Generate shell completion scripts for the current binary.
/// On Windows, generates PowerShell completion.
/// On Linux, generates Zsh, Bash, and Fish completions.
/// Scripts are written to the `OUT_DIR` (or `target/` if `OUT_DIR` is not set).
///
/// # Example
/// ```
/// // Typically called from a build script (`build.rs`):
/// build_comp_scripts().unwrap();
/// // Or, to specify a custom binary name:
/// build_comp_scripts_with_bin_name("myapp").unwrap();
/// ```
pub fn build_comp_scripts() -> Result<(), std::io::Error> {
    let bin_name = env!("CARGO_PKG_NAME");
    build_comp_scripts_with_bin_name(bin_name)
}

/// Generate shell completion scripts for a given binary name.
/// On Windows, generates PowerShell completion.
/// On Linux, generates Zsh, Bash, and Fish completions.
/// Scripts are written to the `OUT_DIR` (or `target/` if `OUT_DIR` is not set).
///
/// # Example
/// ```
/// // Generate completion scripts for "myapp"
/// build_comp_scripts_with_bin_name("myapp").unwrap();
/// ```
pub fn build_comp_scripts_with_bin_name(name: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        build_comp_script(&ShellFlag::Powershell, name)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        build_comp_script(&ShellFlag::Zsh, name)?;
        build_comp_script(&ShellFlag::Bash, name)?;
        build_comp_script(&ShellFlag::Fish, name)?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        build_comp_script(&ShellFlag::Zsh, name)?;
        build_comp_script(&ShellFlag::Bash, name)?;
        build_comp_script(&ShellFlag::Fish, name)?;
        Ok(())
    }
}

fn build_comp_script(shell_flag: &ShellFlag, bin_name: &str) -> Result<(), std::io::Error> {
    let (tmpl_str, ext) = get_tmpl(shell_flag);
    let mut tmpl = just_template::Template::from(tmpl_str);
    tmpl_param!(tmpl, bin_name = bin_name);
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.join("../../../").to_path_buf();
    let output_path = target_dir.join(format!("{}_comp{}", bin_name, ext));
    std::fs::create_dir_all(&target_dir)?;
    std::fs::write(&output_path, tmpl.to_string())
}

fn get_tmpl(shell_flag: &ShellFlag) -> (&'static str, &'static str) {
    match shell_flag {
        ShellFlag::Bash => (TMPL_COMP_BASH, ".sh"),
        ShellFlag::Zsh => (TMPL_COMP_ZSH, ".zsh"),
        ShellFlag::Fish => (TMPL_COMP_FISH, ".fish"),
        ShellFlag::Powershell => (TMPL_COMP_PWSH, ".ps1"),
        ShellFlag::Other(_) => (TMPL_COMP_BASH, ".sh"),
    }
}
