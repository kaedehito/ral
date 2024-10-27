#![allow(warnings)]

use std::env;
mod download_package;
mod file_paths;
mod install_main;
mod setting;
mod structs;
mod consts;
mod install;
#[macro_use]
mod debug_funcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        match args[1].as_str() {
            "inst" | "install" => {
                let mut install_packages: Vec<String> = Vec::new();
                for inst in args.iter().skip(2) {
                    install_packages.push(inst.to_string());
                }
                install_main::install(install_packages);
            }
            "up" | "update" => {
                todo!();
            }
            "rm" | "remove" => {
                todo!();
            }
            _ => {}
        }
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "help" | "h" => help(""),
            _ => {}
        }
    }
}

fn help(arg: &str) {
    if arg == "" {
        println!(
            r#"
ral - Rails of Package Management

Usage:
    ral [options] [package1] [package2] ...

Options:
    install (alias: inst)
        Installs the specified package

        Options:
            --yes or -y
                Returns yes to all questions without indicating yes/no
                No more piping yes commands to ral!

            --no-build or -N
                Disable packages that need to be built
                This means that the only packages that can be installed by ral are those that can be installed by download

            --version
                Installs the specified version of the package
                If you use another package manager to install, this option will be ignored


    upgrade (alias: up)
        Upgrade the specified package

        Options:
            --list or -l
                Update the package list
                This will bring the information in ral's package list up to date

            --all or -a
                Upgrade all currently installed packages
                This will upgrade any packages installed using ral.

            --yes or -y
                Returns yes to all yes/no questions
                This avoids piping the yes command to ral

    remove (alias: rm)
        Removes the specified package
        
            Options:
                --yes or -y
                    Returns yes to all yes/no questions
                    This avoids piping the yes command to ral
    

    help (or h)
        Display this help message

        "#
        );
    }
}
