# Minigrep

A grep clone built in rust based on the [minigrep example](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html) from [the book](https://doc.rust-lang.org/stable/book/title-page.html) along with some of the advice and crates from [this great tutorial](https://rust-cli.github.io/book/index.html). This project and repo are only here for the sake of learning the Rust language.

## Installation

### MacOS / Linux

```sh
$ curl -fsSL https://raw.githubusercontent.com/Steven-Torres/minigrep/main/install.sh | sh
```

### Windows (Powershell)

```pwsh
PS> iwr -URI https://raw.githubusercontent.com/Steven-Torres/minigrep/main/install.ps1 | iex
```

## Updating

```sh
minigrep -u
minigrep --update
```

## Usage

```sh
minigrep <query> [filename]? [FLAGS]
```

- filename can be left out for standard input

## Flags

```
-h, --help
-i, --ignore-case
-n, --no-color
-u, --update
-V, --version
```
