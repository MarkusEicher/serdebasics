use serde::{Deserialize, Serialize};
use serde_json::{Error, to_string_pretty, from_str};

// Building the data structures
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct Bookmark {
    title: String,
    url: String,
    kgdomain: KGDomain,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct KGDomain {
    kgtype: String,
    kgdescription: String,
}

// Serializing (needs to be refactored to a pure main function)
fn main() {
    serialize_bookmarks();
    deserialize_bookmarks();
}

fn serialize_bookmarks() {
    let kgdomain1 = KGDomain {
        kgtype: "Technology".to_string(),
        kgdescription: "Programming Language".to_string(),
    };
    let bookmark1: Bookmark = Bookmark {
        title: "Rust Project Page".to_string(),
        url: "https://www.rust-lang.org/".to_string(),
        kgdomain: kgdomain1,
    };
    let bookmark_ser: Result<String, Error> = to_string_pretty(&bookmark1);
    if bookmark_ser.is_ok() {
        println!("{}", bookmark_ser.ok().unwrap());
    } else {
        println!("{:#?}", bookmark_ser.err());
    }
}

// Deserializing
fn deserialize_bookmarks() {
    let json_string: &str = r#"
    {
        "TITLE": "Rust Project Page",
        "URL": "https://www.rust-lang.org/",
        "KGDOMAIN": {
            "KGTYPE": "Technology",
            "KGDESCRIPTION": "Programming Language"
        }
    }
    "#;

    let bookmark_deserialize = from_str::<Bookmark>(json_string);
    if bookmark_deserialize.is_ok() {
        println!("{:#?}", bookmark_deserialize.ok().unwrap());
    } else {
        println!("{:#?}", bookmark_deserialize.err());
    }
}
