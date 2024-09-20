fn main(){
    println!("cargo::rustc-link-lib=my_math_lib");
    println!("cargo::rustc-link-search=../my_math_lib/target/release/");
}
