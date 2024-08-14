use std::path::PathBuf;

use bpaf::Bpaf;
use depclean::DepClean;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
struct DepCleanOptions {
    /// The path to run this command, will use the current directory if absent.
    path: Option<PathBuf>,
}

fn main() {
    let opts = dep_clean_options().run();
    let mut checker = DepClean::new();
    if let Some(path) = opts.path {
        checker.run_with_path(&path);
    } else {
        checker.run();
    }
}
