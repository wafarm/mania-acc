const DATA: &'static str = include_str!("../data/data.json");

pub fn parse_data() -> serde_json::Value {
    serde_json::from_str(DATA).unwrap()
}
