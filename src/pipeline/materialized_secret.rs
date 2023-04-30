use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize, Clone)]
pub struct MaterializedSecretConfig {
    secrets: Vec<MaterializedSecret>,
    indexed_secrets: HashMap<String, MaterializedSecret>
}


impl MaterializedSecretConfig {
    pub(crate) fn secrets(&self) -> &Vec<MaterializedSecret> {
        &self.secrets
    }

    pub(crate) fn new(materialized_secrets: Vec<MaterializedSecret>) -> MaterializedSecretConfig {
        let mut idx_sec = HashMap::new();

        for mat_sec in materialized_secrets.iter() {
            idx_sec.insert(mat_sec.object_name.clone(), mat_sec.clone());
        }

        MaterializedSecretConfig { secrets: materialized_secrets, indexed_secrets: idx_sec }
    }

    pub(crate) fn indexed_secrets(&self) -> &HashMap<String, MaterializedSecret> {
        &self.indexed_secrets
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct MaterializedSecret {
    #[serde(rename = "objectName")]
    pub object_name: String,
    #[serde(rename = "secretPath")]
    pub secret_path: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}
impl MaterializedSecret {
    pub(crate) fn new(name: String, location: String, key: String) -> MaterializedSecret {
        MaterializedSecret { object_name: name, secret_path: location, secret_key: key }
    }
}