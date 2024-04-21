# Rust Cli

## Install

```sh
# local crates install, change app root. add to $PATH
cargo install --path .

# Or local test
cargo run -- --help

# help
shr --help

# command detail help
shr csv --help

```

## CSV Transformer

一个简单的 CSV 格式转换工具, 支持 Json, Yaml

### Usage

```sh
# help
shr csv --help

# simple default json
shr csv -i ./assets/customers-100.csv

# different format
shr csv -i ./assets/customers-100.csv --format yaml

# change output directory
shr csv -i ./assets/customers-100.csv --outdir dist

```

## Password Generator

一个简单的密码生成工具

### Usage

```sh
# simple
shr gp

```
