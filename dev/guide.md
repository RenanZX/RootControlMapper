# Contributing to the project

If you want to contribute to improving the code:
1. Download the project from GitHub, or fork this repository.
2. Never use the main branch (`main`); create a branch with your changes and open a `pull request` for review.
3. Branch naming follows the `snake_case` pattern; you can use the `git flow` tool to organize your development workflow.
4. Make your changes following the standards established in this document.

# Code Style/Standard

This project adopts the best coding practices in Rust, which are:
- `snake_case`: For functions and variables.
- `PascalCase`: For structs.

# Compilation and Execution

### Installing Language Dependencies

First, install the necessary tools for compiling Rust code:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

- Press 1 and hit enter to install the tool.
- Then, update your environment:

```shell
source "$HOME/.cargo/env"
```

- To verify the installation, just type the following command in the terminal:

```shell
rustc --version
```

You can learn more on the [official language page]("https://rust-lang.org/tools/install/").

### Preparing the Environment

At the root of the project, run the following command in the terminal:

```shell
./build.sh --debug
```

This will create the entire environment necessary to perform tests. If you want to test macros or scripts you developed, use the `environment/debug` folder to make your changes.

### Running the Project

Type the default command in the terminal to execute apps in development with Rust:


```shell
./build.sh --debug
```

### Compiling the Project

- At the root of the project, execute the script in the terminal:

```shell
./build.sh --release
``` 

- A `build` folder will be created containing the project executable and the installation script.
- To create the `.tar` file, just type `./build.sh --release tar` or, if you have already built it, `./build.sh --tar`.
- If you want to clear the builds, type `./build.sh --clear`.
- Check the help command, if you need:

```shell
./build.sh --help
```

### How to check logs and run the app in debug mode

##### Shows logs of info level and above (info, warn, error)
`RUST_LOG=info cargo run`

##### Shows everything, including debug and trace (most detailed level)
`RUST_LOG=trace cargo run`

##### Shows logs only from your main module
`RUST_LOG=debug cargo run`
