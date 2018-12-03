use json::JsonValue;

use std::collections::HashMap;

/// Check if `pos`ition is within first word in `text`.
pub fn in_first_word(pos: usize, text: &str) -> bool {
    if let Some(wpos) = text.find(char::is_whitespace) {
        return pos < wpos;
    }
    return true;
}

pub fn hash_map_to_json(map: &HashMap<String, String>) -> JsonValue {
    let mut val = JsonValue::new_object();
    for (key, value) in map {
        val[key] = JsonValue::from(value.clone());
    }
    val
}

pub fn json_obj_to_hash_map(obj: &JsonValue) -> HashMap<String, String> {
    assert!(obj.is_object());
    let mut map = HashMap::new();
    for (key, val) in obj.entries() {
        if let Some(s) = val.as_str() {
            map.insert(key.to_string(), s.to_string());
        }
    }
    map
}

pub fn replace_vars(data: &String, map: &HashMap<String, String>) -> String {
    let mut res = data.clone();
    for (k, v) in map {
        res = res
            .replace(&format!("${}", k), v)
            .replace(&format!("${{{}}}", k), v);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_first_word_beginning() {
        assert!(in_first_word(0, "hello world"));
    }

    #[test]
    fn in_first_word_middle() {
        assert!(in_first_word(2, "hello world"));
    }

    #[test]
    fn in_first_word_end() {
        assert!(in_first_word(4, "hello world"));
    }

    #[test]
    fn in_first_word_on_boundary_whitespace() {
        assert!(!in_first_word(5, "hello world"));
    }

    #[test]
    fn in_first_word_next_word() {
        assert!(!in_first_word(6, "hello world"));
    }

    #[test]
    fn test_hash_map_to_json() {
        let mut map = HashMap::new();
        map.insert("one".to_string(), "1".to_string());
        map.insert("two".to_string(), "2".to_string());
        map.insert("three".to_string(), "3".to_string());

        let jmap = hash_map_to_json(&map);
        assert!(jmap.is_object());
        assert!(jmap.has_key("one"));
        assert_eq!(jmap["one"], JsonValue::String("1".to_string()));
        assert!(jmap.has_key("two"));
        assert_eq!(jmap["two"], JsonValue::String("2".to_string()));
        assert!(jmap.has_key("three"));
        assert_eq!(jmap["three"], JsonValue::String("3".to_string()));
    }

    #[test]
    fn test_json_obj_to_hash_map() {
        let obj = json::object![
            "one" => "1",
            "two" => "2",
            "three" => "3",
        ];

        let map = json_obj_to_hash_map(&obj);
        assert_eq!(map.len(), 3);
        assert!(map.contains_key("one"));
        assert_eq!(map.get("one"), Some(&"1".to_string()));
        assert!(map.contains_key("two"));
        assert_eq!(map.get("two"), Some(&"2".to_string()));
        assert!(map.contains_key("three"));
        assert_eq!(map.get("three"), Some(&"3".to_string()));
    }

    #[test]
    fn test_replace_vars() {
        let input = String::from("$ONE, ${TWO}, $ONE, $THREE");
        let mut vars = HashMap::new();
        vars.insert("ONE".to_string(), "1".to_string());
        vars.insert("TWO".to_string(), "2".to_string());
        let output = replace_vars(&input, &vars);
        assert_eq!(output, "1, 2, 1, $THREE".to_string());
    }
}
