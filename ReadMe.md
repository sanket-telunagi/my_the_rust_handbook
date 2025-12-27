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
    # Add new chapters here as you create them
]
```

this will create a command target folder, saving disk space

- to run the command "cargo run", go to that chapter directory and run the command, as cargo automatically identifies the main file to run
