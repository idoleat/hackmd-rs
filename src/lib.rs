use reqwest;
use std::collections::HashMap;

struct header {}

fn initialize() {}

pub fn get_note(note_id: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut note =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    if note.len() == 1 {
        Ok(note)
    } else {
        Err("Bad Request".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let result = get_note(String::from("7n8KZy0oQDKQrhwCOhi0KQ"));
        println!("{:#?}", result); // std out seems useless in unit test but I don't neet assert here
        Ok(result.expect("It may panic")) // test will fail if got Err in get_note
    }
}
