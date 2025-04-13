use biblatex::Entry;
use regex::Regex;

#[derive(Debug)]
pub struct Template(String);

impl Template {
    pub fn from(string: String) -> Self {
        Self(string)
    }
    pub fn render(self: &Self, values: &Entry) -> String {
        let re = Regex::new(r"\{\{(\w+)\}\}").expect("Failed to build regex");
        re.replace_all(self.0.as_str(), |caps: &regex::Captures<'_>| {
            let key = &caps[1];
            let val: String = match key {
                "key" => values.key.clone(),
                "type" => values.entry_type.to_string(),
                _ => {
                    let out: String = values.get_as(key).to_owned().unwrap_or_default();
                    out
                }
            };
            val
        })
        .to_string()
    }
}
