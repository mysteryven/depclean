use std::{cell::RefCell, fs, path::Path, rc::Rc, sync::mpsc::Sender};

use fxhash::FxHashSet;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::{AstNodes, Semantic, SemanticBuilder};
use oxc_span::{CompactStr, SourceType};

use crate::{
    checkers::{get_js_checkers, Checker},
    Atom,
};

const EXTENSIONS: [&str; 4] = ["js", "jsx", "mjs", "cjs", "ts", "tsx"];

pub struct DepCheckerContext<'a> {
    semantic: Rc<Semantic<'a>>,
    used_deps: RefCell<FxHashSet<Atom>>,
}

impl<'a> DepCheckerContext<'a> {
    pub fn new(semantic: Rc<Semantic<'a>>) -> Self {
        Self {
            semantic,
            used_deps: RefCell::new(FxHashSet::default()),
        }
    }

    pub fn semantic(&self) -> &Rc<Semantic<'a>> {
        &self.semantic
    }

    pub fn nodes(&self) -> &AstNodes<'a> {
        self.semantic.nodes()
    }

    pub fn module_records(&self) -> &oxc_semantic::ModuleRecord {
        self.semantic().module_record()
    }

    pub fn add_use(&self, name: CompactStr) {
        self.used_deps.borrow_mut().insert(name);
    }

    pub fn into_deps(self) -> Vec<CompactStr> {
        self.used_deps.borrow().iter().cloned().collect()
    }
}

#[derive(Debug, Default)]
pub struct Runtime;

impl Runtime {
    pub fn process_path(&self, path: &Path, sender: &Sender<Vec<CompactStr>>) {
        if path
            .extension()
            .map_or(false, |ext| EXTENSIONS.contains(&ext.to_str().unwrap()))
        {
            let used_deps = self.check_js_files(path);
            sender.send(used_deps).unwrap();
        }
    }

    /// # Panics
    /// if the file extension is not one of "js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx"
    ///
    /// check js kind files, includes .js, .jsx, .ts, .tsx
    /// analyze their esm and cjs dependencies
    pub fn check_js_files(&self, path: &Path) -> Vec<CompactStr> {
        let Ok(source_type) = SourceType::from_path(path) else {
            eprintln!("Unsupported file type: {:?}", path);
            return vec![];
        };

        let source_text = fs::read_to_string(path).unwrap();
        let allocator = Allocator::default();
        let ret = Parser::new(&allocator, &source_text, source_type)
            .allow_return_outside_function(true)
            .parse();
        let program = allocator.alloc(ret.program);
        let semantic_builder = SemanticBuilder::new(&source_text, source_type)
            .build_module_record(path.to_path_buf(), program);
        let semantic_ret = semantic_builder.build(program);

        let ctx = DepCheckerContext::new(Rc::new(semantic_ret.semantic));
        let checkers = get_js_checkers();
        for checker in checkers {
            checker.run(&ctx)
        }

        ctx.into_deps()
    }
}
