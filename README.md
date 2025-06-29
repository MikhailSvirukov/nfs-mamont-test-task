### Тестовое задание для проекта NFS-Mamont

Реализован базовый интерфейс для взаимодействия с in-memory файловой системой  
В качестве файловой системы используется структура:  
``` 
pub struct FileSystem {
    pub hashtable: HashMap<String, Mutex<File>> 
} 
```
С помощью трейта ```FileSystemInterface``` реализован интерфейс:
```
async fn touch(&mut self, path: &str)-> Result<(), Error>;
async fn write(&mut self, path: &str, offset: usize, data: &[u8]) -> Result<(), Error>;
async fn read(&mut self, path: &str, offset: usize, len: usize) -> Result<Vec<u8>, Error>;
```

Также реализован базовый набор тестов для проверки на возвращаемый результат и ошибки
