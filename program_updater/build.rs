#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("zzmt.ico");
    if let Err(e) = res.compile() {
        println!("cargo:warning=无法设置图标: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {
    //非Windows平台提供空的主函数
}
