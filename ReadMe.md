git remote add origin https://github.com/sanket-telunagi/my_the_rust_handbook.git
git branch -M main
git push -u origin main

command looks inside subfolders and deletes the directory .git
windows : Get-ChildItem -Path .\chapter\_\*\.git -Directory -Force | Remove-Item -Recurse -Force
linux : find . -mindepth 2 -name .git -type d -exec rm -rf {} +

creating projects without git
cargo new chapter_05_structs --vcs none

The --vcs none flag ensures it creates the files but skips the .git folder creation.

creating rust workspace

add the following code to cargo toml file to create the workspace

```toml
[workspace]
resolver = "2"
members = [
    "chapter_01_hello",
    "chapter_02_guessing_game",
    "chapter_03_the_basics",
    "chapter_04_ownership",
    "chapter_05_structs",
    "chapter_06_enums"
]
```

this will create a command target folder, saving disk space

- to run the command "cargo run", go to that chapter directory and run the command, as cargo automatically identifies the main file to run

### ch.07.rust_package_system

why package system

- manange & encapsulate code
- module system,
  package --> crates --> (binary, library --> modules)

binary crates :

- each file is itself a binary crate
- defining more binary crates :

  ```txt
  --> bin
      |--> <crate name>.rs (each file will have one binary crate)
  ```

new package defining

```shell
cargo new <package name>
```

Rules for the packages :

- package must have atleast one crate
- package can have zero or one library crate
- pakcage can have any number of binary crate

# library crates :

creating a new library

```shell
cargo new <library name> --lib
```

lib.rs --> library crate file

- each file can only have either 0 or 1 library crate
- if this is inside root directory of source, rust automatically creates the library crate

how to access the libary crate :

```rust
pub fn eat_at_restaurant () {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist() ;
}

```

Modules :
defined by "mod" keyword

```rust
mod module_1 {
    fn fn1 () {
        // function definition
    }

    fn fn2 () {
        // function definition
    }
}
```

keywords in rust :

- pub :

  - by default everything in rust is private, use this keyword to make the function public
  - this can be used to make any part of the code public like function, modules, structs etc

- use :

  - keyword to bring a module inscope

- self :

  - referencing the current module

  ```rust
  use std::io::Result as ioResult ;
  ```

  - referencing multple

  ```rust
  use rand::{
    Rng,
    CryptoRng,
    Errorking::Transient
  };
  ```

nested paths :

- define the module in library but keep the implementation in another binary crate

  ```rust
  mod front_of_the_house;
  ```

  the implementation should have the same name as module

  child modules :

  - crate the directory for child modules
    - child module
