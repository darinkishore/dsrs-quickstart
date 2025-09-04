use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Example {
    pub data: HashMap<String, Value>,
    pub input_keys: Vec<String>,
    pub output_keys: Vec<String>,
}

impl Example {
    pub fn new(
        data: HashMap<String, Value>,
        input_keys: Vec<String>,
        output_keys: Vec<String>,
    ) -> Self {
        let output_keys = if !output_keys.is_empty() {
            output_keys
        } else if !input_keys.is_empty() {
            data.keys()
                .filter(|key| !input_keys.contains(key))
                .cloned()
                .collect()
        } else {
            vec![]
        };

        Self {
            data,
            input_keys,
            output_keys,
        }
    }

    pub fn get(&self, key: &str, default: Option<&str>) -> Value {
        self.data
            .get(key)
            .unwrap_or(&default.unwrap_or_default().to_string().into())
            .clone()
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<Value> {
        self.data.values().cloned().collect()
    }

    pub fn set_input_keys(&mut self, keys: Vec<String>) {
        self.input_keys = keys;

        self.output_keys = self
            .data
            .keys()
            .filter(|key| !self.input_keys.contains(key))
            .cloned()
            .collect();
    }

    pub fn with_input_keys(&self, keys: Vec<String>) -> Self {
        let output_keys = self
            .data
            .keys()
            .filter(|key| !keys.contains(key))
            .cloned()
            .collect();

        Self {
            data: self.data.clone(),
            input_keys: keys,
            output_keys,
        }
    }

    pub fn without(&self, keys: Vec<String>) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|(key, _)| !keys.contains(key))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            input_keys: self
                .input_keys
                .iter()
                .filter(|key| !keys.contains(key))
                .cloned()
                .collect(),
            output_keys: self
                .output_keys
                .iter()
                .filter(|key| !keys.contains(key))
                .cloned()
                .collect(),
        }
    }
}
