use std::{
    env::current_dir,
    path::Path,
    sync::Arc,
};

use colorful::Colorful;
use dependencies::DependenceBuilder;
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
pub struct DepChecker {
    runtime: Arc<Runtime>,
}

impl DepChecker {
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Runtime {}),
        }
    }

    /// Return all unused dependencies below root path.
    ///
    /// # Panics
    /// if package.json not found in the root directory
    pub fn check(&mut self, root_path: &Path) -> Vec<CompactStr> {
        if !root_path.join("package.json").exists() {
            eprintln!("package.json not found in the root directory");
        }
        let paths: FxHashSet<Box<Path>> = Walk::new(root_path.to_path_buf())
            .paths()
            .into_iter()
            .collect();
        let mut builder = DependenceBuilder::new();
        builder.with_package_json(root_path.join("package.json"));
        let dependence_container = builder.build();

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<CompactStr>>();

        paths
            .iter()
            .par_bridge()
            .for_each_with(self.runtime.clone(), |runtime, path| {
                runtime.process_path(path, &sender);
            });

        drop(sender);

        let used_deps = receiver.into_iter().flatten().collect::<FxHashSet<_>>();
        dependence_container
            .unused_dependencies(&used_deps)
            .cloned()
            .collect()
    }

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
        let unused_deps = self.check(path);
        let dep_text = if unused_deps.len() > 1 {
            "dependencies"
        } else {
            "dependence"
        };

        let title = format!("Found {} {} unused", unused_deps.len(), dep_text)
            .bold()
            .blink();
        let body = unused_deps
            .iter()
            .map(|dep| format!("  - {}", dep))
            .collect::<Vec<_>>()
            .join("\n");

        let footer = "\nTo remove them, run `...`".bold();

        println!("{}\n{}\n{}", title, body, footer);
    }
}
