use crate::storage_interface::StorageConfigFile;

pub struct StorageConfigFileImpl;

impl StorageConfigFile for StorageConfigFileImpl {
    async fn get_config_file(&self, file_name: &str) -> Result<Option<String>, String> {
        let mut current_dir =
            std::env::current_dir().map_err(|e| format!("Error getting current dir: {}", e))?;

        loop {
            let file_path = current_dir.join(file_name);

            if file_path.exists() {
                let file_contents = std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Error reading file: {}", e))?;

                return Ok(Some(file_contents));
            }

            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                break;
            }
        }

        Ok(None)
    }
}
