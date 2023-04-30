use serde_json::{json, Map};

use crate::pipeline::{MaterializedSecretConfig, MaterializedSecret};

pub struct VaultAnnotations {
    role: String,
    secrets: Vec<MaterializedSecret>,
}

impl VaultAnnotations {
    pub fn to_serde_values(&self) -> serde_json::Value {
        let mut sub_values: Map<String, serde_json::Value> = Map::new();
        
        sub_values.insert("vault.hashicorp.com/agent-inject".to_string(), json!("true"));
        sub_values.insert("vault.hashicorp.com/role".to_string(), json!(self.role));

        for secret in &self.secrets {
            sub_values.insert(format!("vault.hashicorp.com/agent-inject-secret-{}", secret.object_name), json!(secret.secret_path));
            sub_values.insert(format!("vault.hashicorp.com/agent-inject-template-{}", secret.object_name), 
            json!(format!("
            {{{{ with secret \"constructum/{}\" -}}}}
                export {}=\"{{{{ .Data.data.{} }}}}\"
            {{{{- end }}}}", secret.secret_path, secret.object_name.to_uppercase(), secret.secret_key)));
        }

        serde_json::Value::Object(sub_values)
    }

    pub fn to_source_commands(&self) -> Vec<String> {
        let mut src_commands = vec![];

        for secret in &self.secrets {
            src_commands.push(format!(". /vault/secrets/{}", secret.object_name));
        }
        src_commands
    }
}

pub fn build_vault_annotations(secret_cfg: MaterializedSecretConfig) -> VaultAnnotations {
    let materialized_secret_data = secret_cfg.secrets();
    VaultAnnotations { role: String::from("constructum"), secrets: materialized_secret_data.clone() }
}