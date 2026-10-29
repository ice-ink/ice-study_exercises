use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}

fn main() {
    println!(
        "{:?}",
        group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut result = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        for group in &mut result {
            group.sort();
        }
        result.sort();

        assert_eq!(
            result,
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
            ]
        );

        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec!["".to_string()]]);
        assert_eq!(group_anagrams(vec!["a".to_string()]), vec![vec!["a".to_string()]]);
    }
}
