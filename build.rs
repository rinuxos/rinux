use config;
fn main() {
    if config::PROJECT_NAME == "MyProject" && config::PROJECT_VERSION == "v0.1.0" {
        println!("cargo:rustc-cfg=pureOS");
    } else {
        println!("cargo:rustc-cfg=customOS");
    };
}