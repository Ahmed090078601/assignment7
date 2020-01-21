# First Welcome assignment7 modules
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`assignment7 = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["ahmed090078601 <code.ahmedqamar15@gmail.com>"]
edition = "2018"

[dependencies]
assignment7 = "0.1.0"
```

In `src/main.rs` you can use like this:

```
use assignment7;
fn main() {
    println!("Hello, world!");
    human_resources::software_developer_attendance::time();
}
```
following will also work:
```
use human_resources::software_developer_attendance::time();
fn main() {
    human_resources::software_developer_attendance::time();
    hr_table::food::kfc::burger();
    crate::lib::officals::staff::member ::int();

    println!("Hello, world!");
    piaic();
    }
```

now `cargo run` for results
