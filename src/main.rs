use walkdir::WalkDir;



fn main() {
  let res = walk_get_include_file_list("/Volumes/SSD/study/rust/napi/napi_fs/src".to_owned());
  println!("{:?}", res);
}

pub fn walk_get_include_file_list(current_dir: String) -> Vec<FileData> {
  let mut file_data_list: Vec<FileData> = vec![];
  for entry in WalkDir::new(&current_dir) {
    let entry = entry.unwrap();
    
    let filename = if let Some(name) = entry.file_name().to_str() {
      name.to_owned()
    } else {
      continue
    };

    let is_file = entry.path().is_file();
    file_data_list.push(FileData {
      filename,
      absolute_path: entry.path().display().to_string(),
      is_file,
      size: if is_file {
        if let Ok(metadata) = entry.path().metadata() {
          metadata.len().try_into().unwrap()
        } else {
          0
        }
      } else {
        0
      }
    });
  }
  file_data_list
}

#[derive(Debug)]
pub struct FileData {
  pub filename: String,
  pub absolute_path: String,
  pub size: i64,
  pub is_file: bool,
}