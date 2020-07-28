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


#[test]
fn serialize_struct() {

    let item = Item {
        name: String::from("test"),
        sub: SubItem {
            id: 12
        }
    };

    let mut file = std::fs::File::create("/tmp/serde_tar-test.tar").unwrap();
    
    assert_eq!(serde_tar::to_writer(&mut file, &item), Ok(()));
}