use just_fmt::snake_case;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "general_renderer", derive(serde::Serialize))]
pub enum ShellFlag {
    #[default]
    Bash,
    Zsh,
    Fish,
    Powershell,
    Other(String),
}

impl From<String> for ShellFlag {
    fn from(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "zsh" => ShellFlag::Zsh,
            "bash" => ShellFlag::Bash,
            "fish" => ShellFlag::Fish,
            "pwsh" | "ps1" | "powershell" => ShellFlag::Powershell,
            other => ShellFlag::Other(snake_case!(other)),
        }
    }
}

impl From<ShellFlag> for String {
    fn from(flag: ShellFlag) -> Self {
        match flag {
            ShellFlag::Zsh => "zsh".to_string(),
            ShellFlag::Bash => "bash".to_string(),
            ShellFlag::Fish => "fish".to_string(),
            ShellFlag::Powershell => "powershell".to_string(),
            ShellFlag::Other(s) => s,
        }
    }
}
