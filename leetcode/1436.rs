use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let origins = paths.iter().map(|path| &path[0]).collect::<HashSet<&String>>();

        for path in &paths {
            if !origins.contains(&path[1]) {
                return path[1].clone();
            }
        }
        "".to_string()
    }
}