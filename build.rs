const CARGO_RUSTC_LINK_SEARCH_NATIVE_C: &str = "cargo:rustc-link-search=native=./c";
const CARGO_RUSTC_LINK_LIB_STATIC_CLUA: &str = "cargo:rustc-link-lib=static=clua";
const CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA: &str = "cargo:rustc-link-search=native=./lua-5.4.6";
const CARGO_RUSTC_LINK_LIB_STATIC_LUA: &str = "cargo:rustc-link-lib=static=lua";

fn main() {
    println!("{}", CARGO_RUSTC_LINK_SEARCH_NATIVE_C);
    println!("{}", CARGO_RUSTC_LINK_LIB_STATIC_CLUA);
    println!("{}", CARGO_RUSTC_LINK_SEARCH_NATIVE_LUA);
    println!("{}", CARGO_RUSTC_LINK_LIB_STATIC_LUA);
}