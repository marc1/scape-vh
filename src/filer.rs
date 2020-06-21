use std::{
    fs::DirEntry,
    path::PathBuf
};

use crossterm::Result;

pub struct Dir {
    pub path: PathBuf,
    pub parent_path: Option<PathBuf>,
    pub child_entries: Option<Vec<DirEntry>>,
    //pub selected_entry: Option<DirEntry>,
    pub selected_entry_index: usize
}

impl Dir {
    pub async fn new(path: PathBuf) -> Result<Self> {
        let mut parent_path = None;

        let mut child_entries: Option<Vec<DirEntry>> = Some(
            path.read_dir()? 
            .into_iter()
            .filter_map(|c| c.ok())
            .collect()
        );

        let selected_entry_index = 0usize;

        // TODO: There should be a better way of doing this
        if let Some(_child_entries) = &child_entries {
            if _child_entries.is_empty() {
               child_entries = None; 
            } else {
                // Initializing with zero should be fine because we
                // already determined it isn't empty
                //
                // Maybe we'll deal with this in usage rather than initialization?
                //selected_entry = Some(_child_entries[selected_entry_index]); 
            }
        }

        if let Some(_parent_path) = path.parent() {
            parent_path = Some(_parent_path.to_path_buf());
        }
        

        Ok(Dir { path, parent_path, child_entries, selected_entry_index })
    }
}
