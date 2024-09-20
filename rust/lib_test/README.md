# Rust with library

## Rust build into library

1. create a rust library project: `cargo new --lib my_math_lib`
2. modify Cargo.toml file, specify library type
    ```toml
    [package]
    name = "my_math_lib"
    version = "0.1.0"
    edition = "2021"

    [dependencies]

    [lib]
    crate-type = ["cdylib"]
    ```
3. write your function in the src/lib.rs file
4. compile the library using: `cargo build --release`
    - `release` is for better performance of this library.
    - the `libmy_math_lib.so` library will be in folder `target/release`.

## Rust use library

### mothod 1: use `rustc` to compile project
1. create a rust bin project: `cargo new my_app`
2. define that you want to use fucntions which defined in external library
    ```rust
    extern "C" {
    pub fn add(left: i32, right: i32) -> i32; // function definition in the library
    }
    fn main() {
        println!("Hello, world!");

        let a = 20;
        let b = 300;
        unsafe {
            let result = add(a, b);
            println!("result is {}", result);
        }
    }
    ```
    - define the function
    - call the function in main
3. compile the project using rustc: `rustc src/main.rs -l my_math_lib -L../my_math_lib/target/release`
    - `-l` tells rustc which library to link 
    - `-L` tells rustc where to find the library
4. run the project: `export LD_LIBRARY_PATH=../my_math_lib/target/releas && ./main`
    - as the library is dynamic library, its codes will not be copied to `main` executable. When we execute main, it will try to find add funcion in the `my_math_lib`. We have to use `LD_LIBRARY_PATH` to specify where my_math_lib is.

### method 2: use `cargo` to compile project
1. create a rust bin project: `cargo new my_app`
2. use `build.rs` file to tell Cargo where to find the library, and which library to link to.
    - create a `build.rs` file at the root of the project(not at `src` folder)
    - write the following content to build.rs file:
        ```rust
        fn main(){
            println!("cargo::rustc-link-lib=my_math_lib"); // tell cargo which library to link
            println!("cargo::rustc-link-search=../my_math_lib/target/release/"); // tell cargo where to find library
        }
        ```
3. use cargo to run the project: `cargo run`

> Refer to the [Build Script Page](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for more information about rust build script.
