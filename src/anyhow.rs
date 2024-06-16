use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub fn example() {
    let p = get_person();
    match p {
        Ok(v) => println!("ok -> {:?}", v),
        Err(e) => println!("{e}"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}

fn get_person() -> Result<Person> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(data)
        .with_context(|| "Failed to deserialize.".to_string())
        .map_err(|e| anyhow!(e))?;
    Ok(p)
}
