use std::path::PathBuf;

use anyhow::Context;
use cargo::{GlobalContext, core::Workspace};
use clap::Parser;
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    manifest_path: Option<PathBuf>,
    packages: Vec<String>,
}

impl Args {
    pub fn run(self) -> anyhow::Result<()> {
        let ctx = GlobalContext::default()?;
        let root = self
            .manifest_path
            .as_deref()
            .unwrap_or("Cargo.toml".as_ref());
        let root = std::fs::canonicalize(root)?;
        let workspace = Workspace::new(&root, &ctx)?;
        let mut pending: LinkedHashSet<_> = self.packages.into_iter().collect();
        if pending.is_empty() {
            pending.extend(
                workspace
                    .default_members()
                    .map(|package| package.name().to_string()),
            );
        }
        let mut done = LinkedHashMap::new();
        let mut touched = LinkedHashSet::new();
        let packages: LinkedHashMap<_, _> = workspace
            .members()
            .map(|package| (package.name().to_string(), package))
            .collect();
        while let Some(package) = pending.pop_back() {
            touched.insert(package.clone());
            let packages = {
                let package = *packages.get(&package).context("no package in workspace")?;
                package
                    .dependencies()
                    .iter()
                    .map(|dependency| dependency.package_name())
                    .filter(|package| packages.contains_key(package.as_str()))
                    .map(|package| package.to_string())
                    .collect::<LinkedHashSet<_>>()
            };
            let all_done = packages.iter().all(|package| done.contains_key(package));
            if all_done {
                done.insert(package, packages);
                continue;
            }
            let all_touched = packages.iter().all(|package| touched.contains(package));
            if all_touched {
                anyhow::bail!("cycle detected");
            }
            pending.insert(package);
            pending.extend(packages);
        }

        let roots: LinkedHashSet<_> = {
            let all_dependencies: LinkedHashSet<_> = done.values().flatten().collect();
            done.keys()
                .filter(|package| !all_dependencies.contains(package))
                .cloned()
                .collect()
        };
        println!("publish: {}", roots.iter().join(" "));
        for (package, packages) in done {
            println!();
            println!("publish.{package}: {}", packages.iter().join(" "));
            println!("\tcargo publish --no-verify --locked --package {package}");
        }
        Ok(())
    }
}
