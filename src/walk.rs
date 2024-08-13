use std::path::{Path, PathBuf};

use ignore::{ParallelVisitorBuilder, WalkBuilder, WalkParallel};

pub(crate) struct Walk {
    inner: WalkParallel,
}

impl Walk {
    pub fn new(path: PathBuf) -> Self {
        let walker = WalkBuilder::new(path);
        let walk = walker.build_parallel();
        Self { inner: walk }
    }
    pub fn paths(self) -> Vec<Box<Path>> {
        let (sender, receiver) = std::sync::mpsc::channel::<Vec<Box<Path>>>();
        let mut builer = WalkSender { sender };
        self.inner.visit(&mut builer);
        drop(builer);
        receiver.into_iter().flatten().collect()
    }
}

struct WalkSender {
    sender: std::sync::mpsc::Sender<Vec<Box<Path>>>,
}

impl<'s> ParallelVisitorBuilder<'s> for WalkSender {
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 's> {
        Box::new(WalkController::new(self.sender.clone()))
    }
}

struct WalkController {
    paths: Vec<Box<Path>>,
    sender: std::sync::mpsc::Sender<Vec<Box<Path>>>,
}

impl WalkController {
    pub fn new(sender: std::sync::mpsc::Sender<Vec<Box<Path>>>) -> Self {
        Self {
            paths: Vec::new(),
            sender,
        }
    }
}

impl Drop for WalkController {
    fn drop(&mut self) {
        let paths = std::mem::take(&mut self.paths);
        self.sender.send(paths).unwrap();
    }
}

impl ignore::ParallelVisitor for WalkController {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> ignore::WalkState {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_some_and(|ft| !ft.is_dir()) {
                    self.paths
                        .push(entry.path().to_path_buf().into_boxed_path());
                }
                ignore::WalkState::Continue
            }
            Err(_err) => ignore::WalkState::Skip,
        }
    }
}
