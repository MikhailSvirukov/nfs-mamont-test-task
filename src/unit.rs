use std::collections::HashMap;
use std::sync::Mutex;
use crate::funcs::{FileSystem, FileSystemInterface};

#[tokio::test]
async fn simple_read_after_write() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/text.txt").await.unwrap();
    file_system.lock().unwrap().write("/text.txt", 0, b"text").await.unwrap();
    let content=
        file_system.lock().unwrap().read("/text.txt", 0, 4).await.unwrap();
    assert_eq!(content, b"text");
}


#[tokio::test]
async fn three_writes() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/text.txt").await.unwrap();
    file_system.lock().unwrap().write("/text.txt", 0, b"text").await.unwrap();
    file_system.lock().unwrap().write("/text.txt", 0, b"more and more").await.unwrap();
    file_system.lock().unwrap().write("/text.txt", 5, b"haha").await.unwrap();
    let content=
        file_system.lock().unwrap().read("/text.txt", 0, 9).await.unwrap();
    assert_eq!(content, b"more haha");
}

#[tokio::test]
#[should_panic(expected = "file not exist")]
async fn error_no_file() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/text.txt").await.unwrap();
    file_system.lock().unwrap().read("/some.txt", 5,7).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Offset more than file size")]
async fn error_incorrect_offset_read() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/some.txt").await.unwrap();
    file_system.lock().unwrap().write("/some.txt", 0,b"testing").await.unwrap();
    file_system.lock().unwrap().read("/some.txt", 8,5).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Offset more than file size")]
async fn error_incorrect_offset_write() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/some.txt").await.unwrap();
    file_system.lock().unwrap().write("/some.txt", 7,b"testing").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "file already exist")]
async fn error_existed_file() {
    let file_system=
        Mutex::new(FileSystem {hashtable: HashMap::new()});
    file_system.lock().unwrap().touch("/some.txt").await.unwrap();
    file_system.lock().unwrap().touch("/some.txt").await.unwrap();
}