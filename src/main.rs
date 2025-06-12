use crate::blob::Blob;

mod blob;

fn main() {
    let filename = "picture.jpg";
    let hash = Blob::new_to_blob(filename)
        .convert_to_blob();
    println!("Blob hash: {}", hash);
    
    let convertion_path = Blob::new_to_file(hash.as_str())
        .write_to_file(filename);
    println!("File path: {}", convertion_path);
    println!("Done!")
}
