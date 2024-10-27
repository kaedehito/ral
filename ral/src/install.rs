use std::fs::File;
use std::io::{self, BufReader, Write};
use zip::ZipArchive;
use std::path::Path;
use crate::file_paths::{get_ral_build, get_ral_bin};


pub fn install(packagename: String, file: &String) {
    let build = get_ral_build().join(packagename.clone());
    //let (installsh, removesh, execute_toml) = unzip_package(file, packagename).unwrap_or_else(|e| {
    //    eprintln!("{e}");
    //});


}

pub fn unzip_package(file: &String, packagename: String) -> zip::result::ZipResult<()> {
    let zip_file = File::open(file)?;
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;
    #[allow(warnings)]

    for i in 0..archive.len(){
        let mut file_path = get_ral_build();
        let file_path = file_path.clone().join(packagename.clone());
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(&file_path);

        //if file.display().to_string() == "install"{}

        if let Some(parent) = out_path.clone().parent() {
                std::fs::create_dir_all(parent)?;
            }
        let mut outfile = File::create(&out_path)?;
        io::copy(&mut file, &mut outfile)?;
    }
    
    Ok(())
}
