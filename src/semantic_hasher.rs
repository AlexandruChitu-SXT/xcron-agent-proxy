use serde_json::{Value, Map};
use std::collections::HashMap;

/// Traverses a JSON Abstract Syntax Tree (AST) recursively
/// safely applying the semantic compression dictionary without breaking schema structures.
pub struct SemanticHasher {
    dictionary: HashMap<&'static str, &'static str>,
}

impl SemanticHasher {
    pub fn new() -> Self {
        let mut dict = HashMap::new();
        // Semantic Mapping Dictionary
        dict.insert("tools/invoke", "Z1");
        dict.insert("sessions_send", "Z2");
        dict.insert("execute arbitrage", "Z3");
        dict.insert("analyze mempool", "Z4");
        dict.insert("erd1qqqqqqqqqqqqqpgqtzylnzxc20xmd5a9krt8t9l8kndr90rtv7ls639v39", "W1");
        dict.insert("multiversx", "MX");

        Self { dictionary: dict }
    }

    pub fn hash(&self, mut value: Value) -> Value {
        self.traverse(&mut value);
        // Devolvemos el valor intacto estructuralmente pero comprimido semánticamente.
        // Hemos eliminado el "aplanamiento" a Array para evitar que la API de OpenAI devuelva Error 400.
        value
    }

    fn traverse(&self, value: &mut Value) {
        match value {
            Value::String(s) => {
                // Perform exact or substring replacements based on dictionary
                let mut updated = s.clone();
                for (term, hash) in &self.dictionary {
                    if updated.contains(term) {
                        updated = updated.replace(term, hash);
                    }
                }
                if updated != *s {
                    *value = Value::String(updated);
                }
            }
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    self.traverse(v);
                }
            }
            Value::Object(obj) => {
                let mut new_map = Map::new();
                for (k, mut v) in obj.clone() {
                    self.traverse(&mut v);
                    
                    // Comprimimos también el NOMBRE de la clave (key) si está en el diccionario,
                    // reduciendo tokens sin destruir la estructura JSON que exige la API.
                    let mut compressed_key = k.clone();
                    for (term, hash) in &self.dictionary {
                        if compressed_key.contains(term) {
                            compressed_key = compressed_key.replace(term, hash);
                        }
                    }
                    
                    new_map.insert(compressed_key, v);
                }
                *value = Value::Object(new_map);
            }
            _ => {}
        }
    }
}
