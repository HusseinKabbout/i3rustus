use serde_json::{Result, Value};

// AsRef lets us pass any type T that implements AsRef<str>
// FYI: str and String implent that trait
pub fn jsonify<T: AsRef<str>>(json_str: &str) -> Result<Value> {
    // as_ref returns the needed &str
    let value: Value = serde_json::from_str(json_str.as_ref())?;
    Ok(value)
}
