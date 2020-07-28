# serde_tar

tar serializer for Serde

## Usage

Use `serde_xml_rs::from_reader(...)` on any type that implements [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) as following:

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
    let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
        </Project>
    "##;
    let project: Project = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", project);
}
```