// IMPORT
use std::env;
use std::path;

// MAIN
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:warning=Building for target: {}-{}", target_arch, target_os);
    match target_os.as_str() {
        "macos" => configure_macos(&target_arch),
        "linux" => configure_linux(),
        "windows" => configure_windows(),
        _ => println!("cargo:warning=Unknown target OS: {}", target_os),
    }
}

fn configure_macos(target_arch: &str) {
    println!("cargo:warning=Configuring for macOS");
    let homebrew_prefix = match target_arch {
        "aarch64" => "/opt/homebrew",
        "x86_64" => "/usr/local",
        _ => {
            println!("cargo:warning=Unknown macOS architecture: {}", target_arch);
            "/opt/homebrew"
        }
    };
    let openssl_paths = vec![
        format!("{}/opt/openssl@3", homebrew_prefix),
        format!("{}/opt/openssl@1.1", homebrew_prefix),
        format!("{}/opt/openssl", homebrew_prefix),
    ];
    let mut openssl_found = false;
    for path in openssl_paths {
        let lib_path = format!("{}/lib", path);
        let include_path = format!("{}/include", path);
        if path::PathBuf::from(&lib_path).exists() {
            println!("cargo:warning=Found OpenSSL at: {}", path);
            println!("cargo:rustc-env=OPENSSL_DIR={}", path);
            println!("cargo:rustc-env=OPENSSL_LIB_DIR={}", lib_path);
            println!("cargo:rustc-env=OPENSSL_INCLUDE_DIR={}", include_path);
            println!("cargo:rustc-link-search=native={}", lib_path);
            println!("cargo:rustc-link-lib=crypto");
            println!("cargo:rustc-link-lib=ssl");
            openssl_found = true;
            break;
        }
    }
    if !openssl_found {
        println!("cargo:warning=OpenSSL not found in Homebrew locations");
        println!("cargo:warning=Please install OpenSSL: brew install openssl@3");
        println!("cargo:warning=Trying system OpenSSL as fallback...");
        println!("cargo:rustc-link-lib=crypto");
        println!("cargo:rustc-link-lib=ssl");
    }
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

fn configure_linux() {
    println!("cargo:warning=Configuring for Linux");
    let openssl_paths = vec![
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
        "/usr/local/lib64",
        "/usr/local/lib",
    ];
    let mut openssl_found = false;
    for path in openssl_paths {
        if path::PathBuf::from(path).exists() {
            let lib_crypto = format!("{}/libcrypto.so", path);
            let lib_ssl = format!("{}/libssl.so", path);
            
            if path::PathBuf::from(&lib_crypto).exists() || path::PathBuf::from(&lib_ssl).exists() {
                println!("cargo:warning=Found OpenSSL at: {}", path);
                println!("cargo:rustc-link-search=native={}", path);
                openssl_found = true;
                break;
            }
        }
    }
    if !openssl_found {
        println!("cargo:warning=OpenSSL not found in common locations");
        println!("cargo:warning=Install OpenSSL: sudo apt-get install libssl-dev (Debian/Ubuntu)");
        println!("cargo:warning=Or: sudo yum install openssl-devel (RHEL/CentOS)");
    }
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");
}

fn configure_windows() {
    println!("cargo:warning=Configuring for Windows");
    if let Ok(openssl_dir) = env::var("OPENSSL_DIR") {
        println!("cargo:warning=Using OPENSSL_DIR: {}", openssl_dir);
        let lib_path = format!("{}\\lib", openssl_dir);
        println!("cargo:rustc-link-search=native={}", lib_path);
    } else {
        println!("cargo:warning=OPENSSL_DIR not set");
        println!("cargo:warning=Please install OpenSSL and set OPENSSL_DIR environment variable");
        println!("cargo:warning=Or install vcpkg and use: vcpkg install openssl:x64-windows");
    }
    println!("cargo:rustc-link-lib=libcrypto");
    println!("cargo:rustc-link-lib=libssl");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=crypt32");
    println!("cargo:rustc-link-lib=user32");
}
