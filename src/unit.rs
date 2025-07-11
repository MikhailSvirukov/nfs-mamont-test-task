use std::collections::HashMap;
use crate::{FileSystem, FileSystemInterface};

#[tokio::test]
async fn simple_read_after_write() {
    let mut file_system =
        FileSystem {hashtable: HashMap::new()};
    file_system.touch("/text.txt").await.unwrap();
    file_system.write("/text.txt", 0, b"text").await.unwrap();
    let content=
        file_system.read("/text.txt", 0, 4).await.unwrap();
    assert_eq!(content, b"text");
}


#[tokio::test]
async fn three_writes() {
    let mut file_system =
       FileSystem {hashtable: HashMap::new()};
    file_system.touch("/text.txt").await.unwrap();
    file_system.write("/text.txt", 0, b"text").await.unwrap();
    file_system.write("/text.txt", 0, b"more and more").await.unwrap();
    file_system.write("/text.txt", 5, b"haha").await.unwrap();
    let content=
        file_system.read("/text.txt", 0, 9).await.unwrap();
    assert_eq!(content, b"more haha");
}

#[tokio::test]
#[should_panic(expected = "file not exist")]
async fn error_no_file() {
    let mut file_system =
       FileSystem {hashtable: HashMap::new()};
    file_system.touch("/text.txt").await.unwrap();
    file_system.read("/some.txt", 5,7).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Offset more than file size")]
async fn error_incorrect_offset_read() {
    let mut file_system=
        FileSystem {hashtable: HashMap::new()};
    file_system.touch("/some.txt").await.unwrap();
    file_system.write("/some.txt", 0,b"testing").await.unwrap();
    file_system.read("/some.txt", 8,5).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Offset more than file size")]
async fn error_incorrect_offset_write() {
    let mut file_system=
        FileSystem {hashtable: HashMap::new()};
    file_system.touch("/some.txt").await.unwrap();
    file_system.write("/some.txt", 7,b"testing").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "file already exist")]
async fn error_existed_file() {
    let mut file_system = FileSystem {hashtable: HashMap::new()};
    file_system.touch("/some.txt").await.unwrap();
    file_system.touch("/some.txt").await.unwrap();
}

#[tokio::test]
async fn concurrent_actions() {
    let mut file_system = FileSystem {hashtable: HashMap::new()};
    file_system.touch("/some1.txt").await.unwrap();
    file_system.touch("/some2.txt").await.unwrap();
    file_system.touch("/some3.txt").await.unwrap();
    file_system.write("/some1.txt", 0, b"some1").await.unwrap();
    file_system.write("/some2.txt", 0, b"some2").await.unwrap();
    file_system.write("/some3.txt", 0, b"some3").await.unwrap();
    file_system.write("/some2.txt", 2, b"nne").await.unwrap();
    let res2 = file_system.read("/some2.txt", 0, 5).await.unwrap();
    let res1 = file_system.read("/some1.txt", 0, 5).await.unwrap();
    let res3 = file_system.read("/some3.txt", 0, 5).await.unwrap();
    file_system.write("/some2.txt", 2, b"nne").await.unwrap();
    assert_eq!(res2, b"sonne");
    assert_eq!(res1, b"some1");
    assert_eq!(res3, b"some3");
}
