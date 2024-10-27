use serde::{Deserialize, Serialize};
use std::option::Option;
use std::fs;
use crate::file_paths;

pub trait Search{
    fn search_package(&self) -> Option<PackageList>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PackageListEntry{
    pub version: u64,
    pub date: String,
    pub package: Vec<PackageList>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PackageList{
pub    packagename: String,
pub    license: String,
pub    version: String,
pub    another: String,
pub    url: String,
pub    dependencies: Vec<String>,
pub    build: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigToml{
    pub packagemanager: Vec<PackageManager>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PackageManager{
    pub package_manager: String,
    pub install: String,
    pub search: String,
    pub remove: String,
    pub update: String,
    pub upgrade: String,
}

impl PackageListEntry{
    pub fn search_package(&self, package_name: String) -> Option<PackageList>{
        for pkg in self.package.clone(){
            if pkg.packagename == package_name{
                return Option::from(pkg);

            }
        }
        None
        
    }
}

impl Search for String{
    fn search_package(&self) -> Option<PackageList>{
        let pkglist = file_paths::get_ral_packagelist();
        let file = match fs::read_to_string(pkglist){
            Ok(o) => o,
            Err(e) => {
                eprintln!("Failed to read package list.");
                eprintln!("Error: {}",e);
                std::process::exit(1);
            }
        };

        let parsed: PackageListEntry = serde_json::from_str(&file).unwrap_or_else(|e| {
            eprintln!("Failed to read Json");
            eprintln!("{e}");
            std::process::exit(1);
        });

        for s in parsed.package{
            if s.packagename == *self{
                return Option::from(s);
            }
        }

        None
            
    }
}

