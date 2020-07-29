[![Build Status](https://travis-ci.org/resosafe/serde_tar.svg?branch=master)](https://travis-ci.org/github/resosafe/serde_tar)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

# serde_tar

tar serializer for Serde

## Status

Only simple Serialization at the moment: support for Structures, Map<String, T> and primitive types

## Usage


```rust
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_tar;

#[derive(Debug, Serialize)]
struct Item {
    pub name: String,
    pub sub: SubItem,
}

#[derive(Debug, Serialize)]
struct SubItem {
    pub id: i32,
}

fn main() {
     let item = Item {
        name: String::from("test"),
        sub: SubItem {
            id: 12
        }
    };

    let mut file = std::fs::File::create("/tmp/serde_tar-test.tar").unwrap();
    
    serde_tar::to_writer(&mut file, &item).ok();
}
```