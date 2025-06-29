mod unit;

use std::cmp::min;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    FileExist,
    FileNotExist,
    IncorrectOffset
}

#[allow(async_fn_in_trait)]
pub trait FileSystemInterface {
    async fn touch(&mut self, path: &str)-> Result<(), Error>;
    async fn write(&mut self, path: &str, offset: usize, data: &[u8]) -> Result<(), Error>;
    async fn read(&mut self, path: &str, offset: usize, len: usize) -> Result<Vec<u8>, Error>;
}
pub struct File {
    content: Vec<u8>,
    size: usize,
}

pub struct FileSystem {
    pub hashtable: HashMap<String, Mutex<File>>
}


impl FileSystemInterface for FileSystem {
    async fn touch(&mut self, path: &str)-> Result<(), Error> {
        let new_file = File {content: Vec::new(), size: 0};
        match self.hashtable.insert(path.to_string(), Mutex::new(new_file)) {
            Some(_) => panic!("{1}: {:?}", Error::FileExist
                              ,"file already exist"),
            None => Ok(())
        }
    }

    async fn write(&mut self, path: &str, offset: usize, data: &[u8]) -> Result<(), Error> {
        let result= self.hashtable.get_mut(path);
        match result {
            None => {
                panic!("{1}: {:?}", Error::FileNotExist
                    ,"file not exist")
            }
            Some(_) => {
                let res=result.unwrap().get_mut().unwrap();
                if offset > res.size
                {
                    panic!("{1}: {:?}", Error::IncorrectOffset
                           ,"Offset more than file size")
                }
                let r=[res.content[0..offset].to_owned(), Vec::from(data)].concat();
                let new_file = File {content: r, size: offset+data.len()};
                match self.hashtable.insert(path.to_string(), Mutex::from(new_file)) {
                    Some(_) => Ok(()),
                    None => Err(Error::FileNotExist)
                }
            }
        }
        
    }

    async fn read(&mut self, path: &str, offset: usize, len: usize) -> Result<Vec<u8>, Error> {
        let result= self.hashtable.get_mut(path);
        match result {
            None => {
                panic!("{1}: {:?}", Error::FileNotExist
                       ,"file not exist")
            }
            Some(_) => {
                let res=result.unwrap().get_mut().unwrap();
                if offset > res.size
                {
                    panic!("{1}: {:?}", Error::IncorrectOffset
                           ,"Offset more than file size")
                }
                let result = &res.content[offset..min(res.size, offset+len)];
                Ok(result.to_owned())
            }
        }
    }
}