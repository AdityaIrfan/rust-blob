use sha1::{Digest, Sha1};

pub struct Blob{
    path: String,
    content: Vec<u8>
}

const BLOB_DIR: &str = ".objects";
const CONVERTION_DIR: &str = ".convert";

impl Blob{
    pub fn new_to_blob(file_path: &str) -> Self {
        Self{
            path: file_path.to_string(),
            content: Vec::new()
        }
    }
    
    fn read_file(&self)-> Vec<u8> {
        let file_path = format!("../{}", self.path);
        let error_message = format!("File {} not found", self.path);
        std::fs::read(file_path).expect(error_message.as_str())
    }
    
    fn id(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(b"blob");
        hasher.update(self.content.len().to_string().as_bytes());
        hasher.update(b"\0");
        hasher.update(&self.content);
        format!("{}", hex::encode(hasher.finalize()))
    }

    fn write_blob_to_disk(&self, hash: &str) {
        std::fs::create_dir_all(format!("../{}", BLOB_DIR)).unwrap();
        let path = format!("../{}/{}", BLOB_DIR, hash);
        std::fs::write(path, &self.content).unwrap();
    }
    
    pub fn convert_to_blob(&mut self) -> String {
        self.content = self.read_file();
        let id = self.id();
        self.write_blob_to_disk(&id);
        id
    }
    
    pub fn new_to_file(hash: &str) -> Self {
        Self{
            path: hash.to_string(),
            content: Vec::new()
        }
    }
    
    fn read_blob_file(&self) -> Vec<u8>{
        let path = format!("../{}/{}", BLOB_DIR, self.path);
        let error_message = format!("Blob file {} not found", self.path);
        std::fs::read(path).expect(error_message.as_str())
    }
    
    pub fn write_to_file(&mut self, expected_file_name: &str, ) -> String {
        std::fs::create_dir_all(format!("../{}", CONVERTION_DIR)).unwrap();
        self.content = self.read_blob_file();
        let file_path = format!("../{}/{}", CONVERTION_DIR, expected_file_name);
        std::fs::write(&file_path, &self.content).unwrap();
        file_path.as_str().to_string()
    }
}