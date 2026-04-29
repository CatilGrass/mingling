use std::path::PathBuf;

use just_fmt::kebab_case;

pub fn list_namespaces(
    show_trusted: bool,
    show_untrusted: bool,
    show_untagged: bool,
) -> Vec<String> {
    let wdir = working_dir();
    if !wdir.exists() {
        return Vec::new();
    }

    let mut namespaces = Vec::new();
    let entries = match std::fs::read_dir(&wdir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir()
            && let Some(name) = path.file_name()
            && let Some(name_str) = name.to_str()
        {
            // Skip directories starting with a dot
            if name_str.starts_with('.') {
                continue;
            }
            let namespace = name_str.to_string();
            let is_trusted = is_trusted_namespace(namespace.clone());
            let is_untrusted = is_untrusted_namespace(namespace.clone());
            let is_untagged = is_untagged_namespace(namespace.clone());

            if (show_trusted && is_trusted)
                || (show_untrusted && is_untrusted)
                || (show_untagged && is_untagged)
            {
                namespaces.push(namespace);
            }
        }
    }

    namespaces
}

pub fn set_namespace_trusted(namespace: String, trusted: bool) {
    let ndir = namespace_dir(namespace);
    let trusted_file = ndir.join("TRUSTED");
    let untrusted_file = ndir.join("UNTRUSTED");

    if trusted {
        // Create TRUSTED file and remove UNTRUSTED if it exists
        let _ = std::fs::write(&trusted_file, "");
        let _ = std::fs::remove_file(&untrusted_file);
    } else {
        // Create UNTRUSTED file and remove TRUSTED if it exists
        let _ = std::fs::write(&untrusted_file, "");
        let _ = std::fs::remove_file(&trusted_file);
    }
}

pub fn remove_namespace(namespace: String) {
    let ndir = namespace_dir(namespace);
    if ndir.exists() {
        let _ = std::fs::remove_dir_all(&ndir);
    }
}

pub fn working_dir() -> PathBuf {
    dirs::data_dir().unwrap().join("mingling")
}

pub fn namespace_dir(namespace: String) -> PathBuf {
    working_dir().join(kebab_case!(namespace))
}

pub fn is_untrusted_namespace(namespace: String) -> bool {
    let untrusted_file = namespace_dir(namespace).join("UNTRUSTED");
    untrusted_file.exists()
}

pub fn is_trusted_namespace(namespace: String) -> bool {
    let trusted = namespace_dir(namespace).join("TRUSTED");
    trusted.exists()
}

pub fn is_untagged_namespace(namespace: String) -> bool {
    let ndir = namespace_dir(namespace);
    let trusted = ndir.join("TRUSTED");
    let untrusted = ndir.join("UNTRUSTED");
    !trusted.exists() && !untrusted.exists()
}

pub fn bin_dir(namespace: String) -> PathBuf {
    namespace_dir(namespace).join("bin")
}

pub fn comp_dir(namespace: String) -> PathBuf {
    namespace_dir(namespace).join("comp")
}

pub fn exe_path(namespace: String, bin_name_without_ext: String) -> PathBuf {
    if cfg!(target_os = "windows") {
        bin_dir(namespace).join(bin_name_without_ext + ".exe")
    } else {
        bin_dir(namespace).join(bin_name_without_ext)
    }
}
