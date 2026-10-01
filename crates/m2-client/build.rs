fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-search=native=E:/m2dev-client-src-main/build/vendor/libsodium/RelWithDebInfo");
        println!("cargo:rustc-link-lib=static=sodium");
    }
}
