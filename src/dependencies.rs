use std::path;

use fxhash::{FxHashMap, FxHashSet};
use oxc_span::CompactStr;
use serde::Deserialize;

use crate::Atom;

#[derive(Debug, Deserialize)]
pub struct PackageJSON {
    dependencies: FxHashMap<Atom, Atom>,
}

#[derive(Debug, Default, Deserialize)]
pub struct DependenceContainer {
    /// dependencies collect from package.json
    #[serde(default)]
    dependencies: FxHashSet<CompactStr>,
}

impl DependenceContainer {
    pub fn new() -> Self {
        Self {
            dependencies: FxHashSet::default(),
        }
    }
    pub fn unused_dependencies<'a>(
        &'a self,
        used: &'a FxHashSet<CompactStr>,
    ) -> impl Iterator<Item = &CompactStr> {
        self.dependencies.difference(used)
    }
}

pub struct DependenceBuilder {
    package_json: Option<PackageJSON>,
}

impl DependenceBuilder {
    pub fn new() -> Self {
        Self { package_json: None }
    }
    /// # Panics
    ///
    /// if path is not exist or a valid package.json file
    pub fn with_package_json(&mut self, path: path::PathBuf) -> &mut Self {
        let content = std::fs::read_to_string(path).unwrap();
        let content = serde_json::from_str::<serde_json::Value>(&content).unwrap();
        let content = PackageJSON::deserialize(content).unwrap();
        self.package_json = Some(content);
        self
    }
    pub fn build(self) -> DependenceContainer {
        let mut dependencies = FxHashSet::default();
        if let Some(package_json) = self.package_json {
            for (k, _) in package_json.dependencies {
                dependencies.insert(k);
            }
        }
        DependenceContainer { dependencies }
    }
}
