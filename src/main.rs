mod json;

fn main() {
    json::fetch_json(&json::create_link("createsession"));
}
