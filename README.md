# serde_tar

tar serializer for Serde

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

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

fn main() {
    let s = Item {
        name: String::from("test"),
        sub: SubItem {
            id: 12
        }
    };

    
    let project: Project = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", project);
}
```