use core::{MultiShell, SourceId, Dependency, Source, Summary, Manifest};
use util::{CargoResult, CliError, ToUrl, human};
use sources::{PathSource};
use cargo::util::important_paths::{find_root_manifest_for_cwd};
use toml::{Encoder, encode_str};
use std::io::fs::{File};
use serialize::{Encodable};

pub struct AddOptions {
    pub name: String,
    pub url: String,
}

pub enum DependencyOptions {
    Add(AddOptions),
    Remove(String),
}

pub fn dependency(options: DependencyOptions,
                  shell: &mut MultiShell) -> CargoResult<()> {
    match options {
        Add(ops) => add(ops, shell),
        Remove(dependency_name) => remove(dependency_name, shell),
    }
}

fn add(options: AddOptions,
       shell: &mut MultiShell) -> CargoResult<()> {
    //TODO add a path arg
    let manifest_path = try!(find_root_manifest_for_cwd(None));
    let mut source = try!(PathSource::for_path(&manifest_path.dir_path()).map_err(|e| {
        CliError::new(e.description(), 1)
    }));
    try!(source.update().map_err(|err| CliError::new(err.description(), 1)));

    let package = try!(source.get_root_package());
    let mut manifest = package.get_manifest().clone();
    let mut dependencies = manifest.get_dependencies().to_vec(); 

    //TODO allow alternate sources
    let git_url = try!(options.url.to_url().map_err(human));
    //TODO allow tags and other branches
    let source_id = SourceId::for_git(&git_url, "master");
    //TODO allow specific versions
    let new_dependency = try!(Dependency::parse(options.name.as_slice(), None, &source_id));

    dependencies.push(new_dependency);

    let summary = manifest.get_summary();
    let new_summary = try!(Summary::new(summary.get_package_id().clone(), dependencies, summary.get_features().clone()));
  
    let new_manifest = Manifest::new(new_summary, 
                                     manifest.get_targets().to_vec(), 
                                     manifest.get_target_dir().clone(),
                                     manifest.get_doc_dir().clone(),
                                     manifest.get_build().to_vec(),
                                     manifest.get_exclude().to_vec(),
                                     manifest.get_links().map(|s| s.to_string()),
                                     manifest.get_metadata().clone());

    let content = encode_str(&new_manifest);

    println!("{}", content);

    //try!(File::create(&manifest_path).write_str(content.as_slice()));

    Ok(())
}

fn remove(dependency_name: String,
          shell: &mut MultiShell) -> CargoResult<()> {
    Ok(())
}

