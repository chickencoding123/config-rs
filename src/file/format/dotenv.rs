use std::error::Error;
use std::io::Cursor;

use dotenvy::from_read_iter;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub(crate) fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let mut map: Map<String, Value> = Map::new();
    let cursor = Cursor::new(text);

    for item in from_read_iter(cursor) {
        let (key, mut value) = item?;

        let os_env_vars = std::env::vars_os();
        for (os_string_key, os_string_value) in os_env_vars {
            let string_key: String = os_string_key.to_string_lossy().into_owned();
            if string_key == key {
                value = os_string_value.to_string_lossy().into_owned();
            }
        }

        map.insert(key, Value::new(uri, ValueKind::String(value)));
    }

    Ok(map)
}
