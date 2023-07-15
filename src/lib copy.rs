#![deny(clippy::all)]

use std::{fs, io, path::Path, error::Error};
use walkdir::WalkDir;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn get_include_file_list(current_dir: String) -> Vec<FileData> {
  let mut file_data_list: Vec<FileData> = vec![];
  let path = Path::new(&current_dir);
  if path.is_file() {
    return file_data_list;
  }
  let filename = if let Some(name) = path.file_name() {
    if let Some(name) = name.to_str() {
      name.to_owned()
    } else {
      return file_data_list;
    }
  } else {
    return file_data_list;
  };
  file_data_list.push(
    FileData {
      is_file: false,
      size: 0,
      filename,
      absolute_path: path.display().to_string(),
    }
  );
  get_include_file_list_in_dir(&mut file_data_list, Path::new(&current_dir));
  file_data_list
}

fn get_include_file_list_in_dir(list: &mut Vec<FileData>, dir_path: &Path) {
  if let Ok(entries) = fs::read_dir(dir_path) {
    for entry in entries {
      if let Ok(dir_entry) = entry {
        if let Ok(metadata) = dir_entry.metadata() {
          let is_file = metadata.is_file();
          let size = if is_file {
            metadata.len().try_into().unwrap()
          } else {
            0
          };
          let filename = if let Some(name) = dir_entry.path().file_name() {
            if let Some(name) = name.to_str() {
              name.to_owned()
            } else {
              continue;
            }
          } else {
            continue;
          };
          list.push(FileData {
            filename,
            absolute_path: dir_entry.path().display().to_string(),
            is_file,
            size
          });
          if !is_file {
            get_include_file_list_in_dir(list, dir_entry.path().as_path())
          }
        }
      }
    }
  }
}

fn get_file(list: &mut Vec<FileData>, dir_path: &Path) {

}

#[napi]
pub fn walkdir_get_include_file_list(current_dir: String) -> Vec<FileData> {
  let mut file_data_list: Vec<FileData> = vec![];
  for entry in WalkDir::new(&current_dir) {
    let entry = entry.unwrap();

    let filename = if let Some(name) = entry.file_name().to_str() {
      name.to_owned()
    } else {
      continue;
    };

    let is_file = entry.path().is_file();
    file_data_list.push(FileData {
      filename,
      absolute_path: entry.path().display().to_string(),
      is_file,
      size: if let Ok(metadata) = entry.path().metadata() {
        metadata.len().try_into().unwrap()
      } else {
        0
      },
    });
  }
  file_data_list
}

#[napi(object)]
pub struct FileData {
  pub filename: String,
  pub absolute_path: String,
  pub size: i64,
  pub is_file: bool,
}
