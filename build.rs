use std::env;

fn main() {

    // required libraries
    // https://www.infradead.org/openconnect/building.html

    let target = &env::var("TARGET").unwrap();
    match target.as_str() {
        "x86_64-apple-darwin" => {
            // brew install gettext
            println!("cargo:rustc-link-search=/usr/local/opt/gettext/lib");
            println!("cargo:rustc-link-lib=intl");

            // brew install libxml2
            // println!("cargo:rustc-link-search=/usr/local/opt/libxml2/lib");
            // println!("cargo:rustc-link-lib=xml2");

            // /usr/lib/libiconv.dylib
            println!("cargo:rustc-link-lib=iconv");

            // brew install libproxy
            println!("cargo:rustc-link-search=/usr/local/opt/libproxy/lib");
            println!("cargo:rustc-link-lib=proxy");

            // brew install lz4
            // println!("cargo:rustc-link-search=/usr/local/opt/lz4/lib");
            // println!("cargo:rustc-link-lib=lz4");
            
            // brew install openssl
            // println!("cargo:rustc-link-search=/usr/local/opt/openssl/lib");
            // println!("cargo:rustc-link-lib=ssl");

            // println!("cargo:rustc-link-search=/usr/local/opt/p11-kit/lib");
            // println!("cargo:rustc-link-lib=p11-kit");

        },
        _ => (),
    }
}