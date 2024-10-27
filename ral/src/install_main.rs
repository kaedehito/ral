use crate::{
    consts, download_package, file_paths,
    structs::{self, Search},
};
use crate::dprintln;
use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};

#[allow(warnings)]
pub fn install(packages: Vec<String>) {
    let mut package: Vec<HashMap<&str, String>> = Vec::new();
    let mut depen: Vec<String> = Vec::new();
    let mut build = false;
    let packagelist = read_package_list();

    let cfg: structs::PackageListEntry = serde_json::from_str(&packagelist).unwrap_or_else(|e| {
        eprintln!("{}: Failed to parse json format", "Error".red().bold());
        eprintln!("An error has occurred.");
        eprintln!("Error message: {}", e);
        std::process::exit(1);
    });

    if cfg.version > consts::VERSION {
        eprintln!(
            "{}: The Json version in the package list is not the one currently supported by ral",
            "Fatal".red().bold()
        );
        eprintln!(
            "The u64 value in the “version” tag is greater than or equal to the current value."
        );
        eprintln!("To cure this, upgrade the version of ral.");
        std::process::exit(4);
    }

    let mut install_package_list: Vec<structs::PackageList> = Vec::new();

    for pkg in packages {
        let s = cfg.search_package(pkg.clone()).unwrap_or_else(|| {
            println!("Package not found: {}", pkg);
            std::process::exit(1);
        });
        install_package_list.push(s);
    }

    let mut sum: u64 = 0;
    let mut packages: Vec<String> = Vec::new();
    let mut packageversion: Vec<String> = Vec::new();
    for ins in install_package_list.clone() {
        sum += 1;
        packages.push(ins.packagename);
        packageversion.push(ins.version.clone());
        for d in ins.dependencies {
            if d == "" {
                break;
            }
            let s = d.search_package();
            let s = s.unwrap_or_else(|| {
                //TODO
                std::process::exit(1);
            });
            sum += 1;
            packages.push(s.packagename);
            packageversion.push(s.version.clone());
        }
    }
    print!("\nPackages ({sum}) ");

    let mut packageses: Vec<String> = Vec::new();
    sum = 0;

    for (pack, ver) in packages.clone().into_iter().zip(packageversion.clone().into_iter()) {
        let format = format!("{pack}-{ver}");
        packageses.push(format);
    }

    for pack in packageses.clone() {
        sum += 1;
        print!("{pack} ");
        if sum == 3 {
            sum = 0;
            print!("\n");
        }
    }

    io::stdout().flush().unwrap();

    print!("\n\nProceed with installation? [Y/n] ");

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "yes" || input == "y" || input == "" {
        println!();
        for (inst, version) in packages.into_iter().zip(packageversion) {
            let url = format!(
                "https://github.com/kaedehito/cocoa/releases/download/old/cocore"
            );
            if let Err(e) = download_package::download_package(&url, &inst, version){
                eprintln!("{e}");
            }
        }
        println!();
    } else {
        return;
    }
}

pub fn read_package_list() -> String {
    std::fs::read_to_string(file_paths::get_ral_packagelist()).unwrap_or_else(|e| {
        eprintln!("Failed to read package list");
        eprintln!("{e}");
        std::process::exit(1);
    })
}
