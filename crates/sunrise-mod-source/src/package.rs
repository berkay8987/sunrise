//! Which files of a mod folder are its package's source: the one selection that packing
//! (`sunrise_mod_package::Package::directory_files`) and the uses scan ([`crate::mod_uses`]) share.
use std::path::Path;

/// Folders of a mod's project that are never part of its package: Git's, Cargo's build output
/// and the pinned SDK the mod tool fetches into `.sunrise/`.
pub const NOT_PACKAGED: [&str; 3] = [".git", "target", ".sunrise"];

/// A source file that belongs to the flat Cargo project, not its resource packs.
pub fn source_file(name: &str) -> bool {
    if name.split('/').any(|part| NOT_PACKAGED.contains(&part)) {
        return false;
    }
    name == "Cargo.toml"
        || name == "Cargo.lock"
        || name == "build.rs"
        || (name.starts_with("src/") && (name.ends_with(".rs") || name.ends_with(".toml")))
        || (!name.contains('/')
            && ["README", "LICENSE", "LICENCE", "COPYING", "NOTICE"]
                .iter()
                .any(|prefix| name.starts_with(prefix)))
}

/// The source files a package of the folder at `root` carries, `/`-separated and sorted: the
/// root files and the files under `src/` that [`source_file`] names, outside [`NOT_PACKAGED`]
/// folders. Nothing else is read: no folder at the top but `src/`, and no entry whose name a
/// package path cannot hold (not UTF-8, or with a `\` in it). The walk keeps its own stack, so
/// a deep tree needs no recursion.
pub fn source_files(root: &Path) -> std::io::Result<Vec<String>> {
    walk(root, source_file)
}

/// The files under `src/` of the folder at `root` that a package of it leaves out, because
/// [`source_file`] does not name them (`src/catalog.json`), `/`-separated and sorted, outside
/// [`NOT_PACKAGED`] folders. A build stages only the packaged files, so code that embeds one of
/// these (`include_str!("catalog.json")`) builds in the folder but not from its package.
pub fn left_out_source_files(root: &Path) -> std::io::Result<Vec<String>> {
    walk(root, |name| {
        name.starts_with("src/")
            && !source_file(name)
            && !name.split('/').any(|part| NOT_PACKAGED.contains(&part))
    })
}

/// The files of the folder at `root` that `keep` takes: the root files and the files under
/// `src/`, outside [`NOT_PACKAGED`] folders, skipping names a package path cannot hold.
fn walk(root: &Path, keep: impl Fn(&str) -> bool) -> std::io::Result<Vec<String>> {
    let mut found = Vec::new();
    let mut pending = vec![(root.to_path_buf(), String::new())];
    while let Some((dir, prefix)) = pending.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let file_name = entry.file_name();
            let Some(file_name) = file_name.to_str().filter(|name| !name.contains('\\')) else {
                continue;
            };
            let name = if prefix.is_empty() {
                file_name.to_owned()
            } else {
                format!("{prefix}/{file_name}")
            };
            if kind.is_dir() {
                if (prefix.is_empty() && file_name != "src") || NOT_PACKAGED.contains(&file_name) {
                    continue;
                }
                pending.push((entry.path(), name));
            } else if keep(&name) {
                found.push(name);
            }
        }
    }
    found.sort();
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A package carries the root files and the `.rs` and `.toml` files under `src/`; the other
    /// files under `src/` are the ones it leaves out, and nothing outside `src/` or in a
    /// not-packaged folder is either.
    #[test]
    fn a_data_file_under_src_is_left_out_of_the_package() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for name in [
            "Cargo.toml",
            "README.md",
            "notes.json",
            "src/lib.rs",
            "src/catalog.rs",
            "src/catalog.json",
            "src/data/table.toml",
            "src/data/icon.png",
            "src/target/build.json",
            "assets/demo/lang/en_us.json",
        ] {
            let path = root.join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, b"").unwrap();
        }
        assert_eq!(
            source_files(root).unwrap(),
            [
                "Cargo.toml",
                "README.md",
                "src/catalog.rs",
                "src/data/table.toml",
                "src/lib.rs"
            ]
        );
        assert_eq!(
            left_out_source_files(root).unwrap(),
            ["src/catalog.json", "src/data/icon.png"]
        );
    }
}
