use std::{path, str::FromStr};

use fxhash::{FxHashMap, FxHashSet};
use oxc_span::CompactStr;
use serde::Deserialize;

use crate::Atom;

#[derive(Debug, Deserialize)]
pub struct PackageJSON {
    #[serde(default)]
    dependencies: FxHashMap<Atom, Atom>,
}

#[derive(Debug, Default, Deserialize)]
pub struct PackageJSONContainer {
    dependencies: FxHashSet<CompactStr>,
    unused_dependencies: FxHashSet<CompactStr>,
}

impl PackageJSONContainer {
    pub fn new() -> Self {
        Self {
            dependencies: FxHashSet::default(),
            unused_dependencies: FxHashSet::default(),
        }
    }

    pub fn unused_dependencies(&self) -> &FxHashSet<CompactStr> {
        &self.unused_dependencies
    }

    pub fn compute_unused_deps<'a>(&'a mut self, used: &'a FxHashSet<CompactStr>) {
        self.unused_dependencies = self
            .dependencies
            .iter()
            .filter_map(|dep| {
                // <https://github.com/npm/npm/issues/7351#issuecomment-74307522>
                // If our package.json contains a dependency named "dep"
                // 1. import Dep from 'dep'
                if used.contains(dep) {
                    return None;
                }

                // 2. import { dep1 } from 'dep/dep1'
                for used_dep in used.iter() {
                    if used_dep.starts_with(&format!("{dep}/")) {
                        return None;
                    }
                }

                Some(dep.clone())
            })
            .collect();
    }
}

pub struct PackageJSONBuilder {
    package_json: Option<PackageJSON>,
    /// The raw content of package.json
    raw: String,
}

impl PackageJSONBuilder {
    pub fn new() -> Self {
        Self {
            package_json: None,
            raw: String::new(),
        }
    }
    /// # Panics
    ///
    /// if path is not exist or a valid package.json file
    pub fn with_package_json(&mut self, path: &path::Path) -> &mut Self {
        let raw = std::fs::read_to_string(path).unwrap();
        let content = serde_json::from_str::<serde_json::Value>(&raw).unwrap();
        let content = PackageJSON::deserialize(content).unwrap();
        self.package_json = Some(content);
        self.raw = raw;
        self
    }
    pub fn build(self) -> PackageJSONContainer {
        let mut dependencies = FxHashSet::default();
        if let Some(package_json) = self.package_json {
            for (k, _) in package_json.dependencies {
                dependencies.insert(k);
            }
        }
        PackageJSONContainer {
            dependencies,
            unused_dependencies: FxHashSet::default(),
        }
    }
}

#[derive(Default)]
pub enum PkgManager {
    #[default]
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PkgManager {
    pub fn new_from_path(path: &path::Path) -> Self {
        let yarn_lock = path.join("yarn.lock");
        let pnpm_lock = path.join("pnpm-lock.yaml");
        let package_lock = path.join("package-lock.json");
        let bun_lock = path.join("bun.lockb");

        if yarn_lock.exists() {
            return PkgManager::Yarn;
        }

        if pnpm_lock.exists() {
            return PkgManager::Pnpm;
        }

        if package_lock.exists() {
            return PkgManager::Npm;
        }

        if bun_lock.exists() {
            return PkgManager::Bun;
        }

        let package_json_path = path.join("package.json");
        let package_json = std::fs::read_to_string(&package_json_path).unwrap();
        let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();
        let package_manager = package_json
            .get("packageManager")
            .and_then(|v| v.as_str())
            .unwrap_or("npm");

        package_manager.parse().unwrap_or_default()
    }

    pub fn get_uninstall_cmd(&self) -> &'static str {
        match self {
            PkgManager::Npm => "npm uninstall",
            PkgManager::Yarn => "yarn remove",
            PkgManager::Pnpm => "pnpm remove",
            PkgManager::Bun => "bun remove",
        }
    }
}

impl FromStr for PkgManager {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "npm" => Ok(PkgManager::Npm),
            "yarn" => Ok(PkgManager::Yarn),
            "pnpm" => Ok(PkgManager::Pnpm),
            "bun" => Ok(PkgManager::Bun),
            _ => Err(()),
        }
    }
}
