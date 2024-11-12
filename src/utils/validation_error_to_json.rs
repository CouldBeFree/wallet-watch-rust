use serde_json::json;
use validator::ValidationErrors;

pub fn validation_errors_to_json(errors: ValidationErrors) -> serde_json::Value {
    let mut error_map = serde_json::Map::new();

    for (field, errors) in errors.field_errors() {
        let messages: Vec<String> = errors.iter()
            .map(|e| e.message.clone().unwrap_or_else(|| "Invalid value".into()).to_string())
            .collect();
        error_map.insert(field.to_string(), json!(messages));
    }

    json!({ "errors": error_map })
}