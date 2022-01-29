use std::path::Path;
use std::ffi::OsStr;

pub struct FileExt {
    extension: String
}

impl FileExt {
    pub fn from<P: AsRef<Path> + AsRef<OsStr> + ?Sized>(path: &P) -> FileExt {
        let path = Path::new(&path);

        let mut path_extension = String::new();
        
        // Extract the extension from a Path into an Option<Some(OsString), None> then into Option<Some(String), None>
        let _ = match path.extension() {
            Some(result) => {
                let _ = match result.to_str() {
                    Some(value) => {
                        path_extension.push('.');
                        path_extension.push_str(&value.to_string());
                    },
                    None => (),
                };
            },
            None => (),
        };

        FileExt { extension: path_extension }
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }
}

fn main() {
    let my_file_1 = FileExt::from("C:\\foo.txt");
    
    let path_2 = String::from("C:\\bar.bat");
    let path_3 = Path::new("C:\\foobar.exe");
    
    let my_file_2 = FileExt::from(&path_2);
    let my_file_3 = FileExt::from(&path_3);

    println!("Extension 1 is: {}", my_file_1.extension());
    println!("Extension 2 is: {}", my_file_2.extension());
    println!("Extension 3 is: {}", my_file_3.extension());
}
