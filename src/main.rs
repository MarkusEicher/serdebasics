use serde::{Deserialize, Serialize};
use serde_json::{Error, to_string_pretty, from_str};
use std::fs;
use std::path::Path;

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

// Main function
fn main() {
    serialize_bookmarks();
    deserialize_bookmarks();
}

// Serializing here
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

// Deserializing here

// Read in json file from /assets
fn deserialize_bookmark_from_file<P: AsRef<Path>>(path: P) -> Result<Bookmark, serde_json::Error> {
    let json_content = fs::read_to_string(path).expect("Datei konnte nicht gelesen werden");
    let bookmark: Bookmark = from_str(&json_content)?;
    Ok(bookmark)
}

// Deserialize the file content
fn deserialize_bookmarks() {
    match deserialize_bookmark_from_file("assets/bookmarks.json") {
        Ok(bookmark) => println!("{:#?}", bookmark),
        Err(e) => eprintln!("Fehler beim Deserialisieren: {}", e),
    }
}


/* fn deserialize_bookmarks() {
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

} */
