use std::{env::current_dir, path::Path, sync::Arc};

use colorful::Colorful;
use dependencies::{PackageJSONBuilder, PkgManager};
use fxhash::FxHashSet;
use oxc_span::CompactStr;
use rayon::iter::{ParallelBridge, ParallelIterator};
use runtime::Runtime;
use walk::Walk;

mod checkers;
mod dependencies;
mod runtime;
mod walk;

use checkers::Checker;

type Atom = CompactStr;

#[derive(Debug, Default)]
pub struct DepClean {
    runtime: Arc<Runtime>,
}

impl DepClean {
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Runtime {}),
        }
    }

    /// Return all used dependencies below root path.
    ///
    /// # Panics
    /// if package.json not found in the root directory
    pub fn check(&mut self, root_path: &Path) -> FxHashSet<CompactStr> {
        let paths: FxHashSet<Box<Path>> = Walk::new(root_path.to_path_buf())
            .paths()
            .into_iter()
            .collect();

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<CompactStr>>();

        paths
            .iter()
            .par_bridge()
            .for_each_with(self.runtime.clone(), |runtime, path| {
                runtime.process_path(path, &sender);
            });

        drop(sender);

        receiver.into_iter().flatten().collect::<FxHashSet<_>>()
    }

    /// Run the checker in the current directory.
    pub fn run(&mut self) {
        let root_path = match current_dir() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        self.run_with_path(&root_path);
    }

    pub fn run_with_path(&mut self, path: &Path) {
        let package_json_path = path.join("package.json");
        if !package_json_path.exists() {
            let text = "Didn't find package.json in the current directory, did you run this command in the right place?".red();
            eprintln!("{text}");
            std::process::exit(1);
        }

        let mut builder = PackageJSONBuilder::new();
        builder.with_package_json(&package_json_path);
        let mut package_json_container = builder.build();
        let used_deps = self.check(path);
        package_json_container.compute_unused_deps(&used_deps);
        let unused_deps = package_json_container.unused_dependencies();

        if unused_deps.is_empty() {
            let text = "No unused dependencies found, Good!".rainbow();
            println!("{:?}", text);
            std::process::exit(0);
        }

        let dep_text = if unused_deps.len() > 1 {
            let title = format!("{} dependencies unused in your project:", unused_deps.len(),);
            let body = unused_deps
                .iter()
                .map(|dep| format!("  - {}", dep))
                .collect::<Vec<_>>()
                .join("\n");
            format!("{title}\n{body}")
        } else {
            format!(
                r#""{}" is unused in your project."#,
                unused_deps.iter().next().unwrap()
            )
        };

        let footer = format!(
            "\nRun `{} {}` to clean your codebase.",
            PkgManager::new_from_path(path).get_uninstall_cmd(),
            unused_deps
                .iter()
                .map(|dep| dep.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .green();

        println!("{dep_text}\n{}", footer);
    }
}
