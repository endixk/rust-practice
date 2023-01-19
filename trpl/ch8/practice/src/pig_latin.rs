pub fn pig_latin(s: &str) -> String {
    let mut ret = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    for word in s.split_whitespace(){
        if vowels.contains(&word.chars().nth(0).unwrap()){
            ret.push_str(&format!("{}-hay ", word));
        } else {
            ret.push_str(&format!("{}-{}ay ", &word[1..], word.chars().nth(0).unwrap()));
        }
    }

    ret.trim_end().to_string()
}