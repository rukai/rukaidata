use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct OutDir {
    path: PathBuf,
}

impl OutDir {
    pub fn new(path: &str) -> Self {
        let path = Path::new("../root").join(path);
        fs::create_dir_all(&path).unwrap();
        OutDir { path }
    }

    pub fn compressed_file_writer(&self, file_name: &str) -> GzEncoder<File> {
        let file = File::create(self.path.join(file_name)).unwrap();
        flate2::write::GzEncoder::new(file, Compression::best())
    }

    pub fn create_compressed_file(&self, file_name: &str, data: &[u8]) -> String {
        let path = self.path.join(file_name);
        let file = File::create(&path).unwrap();
        let mut writer = flate2::write::GzEncoder::new(file, Compression::best()); //TODO: try deflate
        writer.write_all(data).unwrap();
        Path::new("/")
            .join(path.strip_prefix("../root").unwrap())
            .into_os_string()
            .into_string()
            .unwrap()
    }

    pub fn create_file(&self, file_name: &str, data: &[u8]) -> String {
        let path = self.path.join(file_name);
        std::fs::write(&path, data).unwrap();
        Path::new("/")
            .join(path.strip_prefix("../root").unwrap())
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
