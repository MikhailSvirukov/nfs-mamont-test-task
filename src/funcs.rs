use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    FileExist,
    FileNotExist,
    IncorrectOffset
}

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
    pub(crate) hashtable: HashMap<String, File>
}


impl FileSystemInterface for FileSystem {
    async fn touch(&mut self, path: &str)-> Result<(), Error> {
        let new_file = File {content: Vec::new(), size: 0};
        match self.hashtable.insert(path.to_string(), new_file) {
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
                if offset > result.unwrap().size
                {
                    panic!("{1}: {:?}", Error::IncorrectOffset
                           ,"Offset more than file size")
                }
                let r=[self.hashtable[path].content[0..offset].to_owned(), Vec::from(data)].concat();
                let new_file = File {content: r, size: offset+data.len()};
                match self.hashtable.insert(path.to_string(), new_file) {
                    Some(_) => Ok(()),
                    None => Err(Error::FileNotExist)
                }
            }
        }
        
    }

    async fn read(&mut self, path: &str, offset: usize, len: usize) -> Result<Vec<u8>, Error> {
        let result= self.hashtable.get(path);
        match result {
            None => {
                panic!("{1}: {:?}", Error::FileNotExist
                       ,"file not exist")
            }
            Some(_) => {
                if offset > result.unwrap().size
                {
                    panic!("{1}: {:?}", Error::IncorrectOffset
                           ,"Offset more than file size")
                }
                let content = &result.unwrap()
                    .content[offset..min(offset+len, result.unwrap().size)];
                Ok(content.to_owned())
            }
        }
    }
}