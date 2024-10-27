use std::path::Path;

#[allow(warnings)]
pub fn get_ral_home() -> &'static Path{
    Path::new("/home/ral/")
}

#[allow(warnings)]
pub fn get_ral_bin() -> &'static Path{
    Path::new("/home/ral/bin/")
}

pub fn get_ral_build() -> &'static Path{
    Path::new("/home/ral/build/")
}

#[allow(warnings)]
pub fn get_ral_log() -> &'static Path{
    Path::new("/home/ral/log/")
}

#[allow(unused)]
pub fn get_ral_packagelist() -> &'static Path{
    Path::new("/home/ral/packagelist.json")
}
