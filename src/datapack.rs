use std::collections::HashMap;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::json;

use crate::error::CompileError;
use crate::mcfunction::MCFunction;

pub struct DataPack {
    pub pub_namespace: NameSpace,
    pub private_namespace: NameSpace,
    description: String,
    name: String,
}

impl DataPack {
    pub fn new(name: &str, private_namespace_name: &str) -> DataPack {
        DataPack {
            pub_namespace: NameSpace::new(name),
            private_namespace: NameSpace::new(private_namespace_name),
            description: "Compiled from MCFL (Minecraft Function Language)".to_owned(),
            name: name.to_owned(),
        }
    }

    /// Save this datapack to a directory.
    ///
    /// * `dest_dir` - The directory in which to write this datapack. For example, the datapacks folder for a Minecraft save.
    ///
    /// Note that the datapack will be saved to a folder whose title is the name of the datapack.
    pub fn save(&self, dest_dir: &Path) -> Result<(), CompileError> {
        let path = &dest_dir.join(&self.name);

        if path.exists() {
            remove_dir_all(path)?;
        }
        create_dir_all(path)?;

        let mcmeta_path = path.join("pack.mcmeta");
        let mcmeta_content = json!({
            "pack": {
                "pack_format": 4,
                "description": self.description
            }
        });
        let mut mcmeta = File::create(mcmeta_path)?;
        mcmeta.write_all(mcmeta_content.to_string().as_bytes())?;

        let tags_path = path
            .join("data")
            .join("minecraft")
            .join("tags")
            .join("functions");
        create_dir_all(&tags_path)?;
        if self.pub_namespace.functions.contains_key("tick") {
            let tick_path = tags_path.join("tick.json");
            let tick_content = json!({ "values": [format!("{}:tick", self.pub_namespace.id)] });
            let mut tick = File::create(tick_path)?;
            tick.write_all(tick_content.to_string().as_bytes())?;
        }
        if self.pub_namespace.functions.contains_key("startup") {
            let startup_path = tags_path.join("load.json");
            let startup_content =
                json!({ "values": [format!("{}:startup", self.pub_namespace.id)] });
            let mut startup = File::create(startup_path)?;
            startup.write_all(startup_content.to_string().as_bytes())?;
        }

        self.pub_namespace.save(path)?;
        self.private_namespace.save(path)?;

        Ok(())
    }
}

pub struct NameSpace {
    pub id: String,
    pub functions: HashMap<String, MCFunction>,
}

impl NameSpace {
    pub fn new(id: &str) -> NameSpace {
        NameSpace {
            id: id.to_owned(),
            functions: HashMap::new(),
        }
    }

    pub fn save(&self, root: &Path) -> Result<(), CompileError> {
        if !self.functions.is_empty() {
            let func_root_path = root.join("data").join(&self.id).join("functions");
            create_dir_all(&func_root_path)?;

            for (name, func) in &self.functions {
                let func_path = func_root_path.join(format!("{}.mcfunction", name));
                let mut func_file = File::create(func_path)?;
                func_file.write_all(format!("{}", func).as_bytes())?;
            }
        }

        Ok(())
    }
}
