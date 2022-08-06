#![feature(map_try_insert)]

use std::collections::HashMap;

pub fn compare_lines(files: Vec<String>) -> bool {
    // ensure files have the same number of lines
    if !equal_line_counts(&files) {
        return false;
    }

    let mut maps = Vec::with_capacity(files.len());

    for file in files {
        let map = map_lines(file);
        maps.push(map);
    }

    if !maps_equal(&maps) {
        return false;
    }

    true
}

fn map_lines(file_contents: String) -> HashMap<String, i32> {
    let mut line_map = HashMap::new();

    for line in file_contents.lines() {
        let occurences: i32 = *line_map.get(&line.to_string()).unwrap_or(&0);

        line_map.insert(line.to_string(), occurences + 1);
    }

    line_map
}

fn equal_line_counts(files: &[String]) -> bool {
    // it's safe to index into this vec as it will always contain at least two elements
    let line_count = files[0].lines().count();

    for file in files.iter().skip(1) {
        if file.lines().count() != line_count {
            return false;
        }
    }

    true
}

fn maps_equal(maps: &[HashMap<String, i32>]) -> bool {
    let first = &maps[0];

    for map in maps.iter().skip(1) {
        for (key, value) in first {
            if !map.contains_key(key) || map.get(key).unwrap() != value {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::compare_lines;

    // Test two identical files return true
    #[test]
    fn test_compare_lines_set1() {
        let file1_path = Path::new("src/testdata/set1/file1.txt");
        let file2_path = Path::new("src/testdata/set1/file2.txt");

        let file1 = fs::read_to_string(file1_path).unwrap();
        let file2 = fs::read_to_string(file2_path).unwrap();

        assert!(compare_lines(vec![file1, file2]));
    }

    // Test two files with identical lines in different places returns true
    #[test]
    fn test_compare_lines_set2() {
        let file1_path = Path::new("src/testdata/set2/file1.txt");
        let file2_path = Path::new("src/testdata/set2/file2.txt");

        let file1 = fs::read_to_string(file1_path).unwrap();
        let file2 = fs::read_to_string(file2_path).unwrap();

        assert!(compare_lines(vec![file1, file2]));
    }

    // Test files with difference lengths are not equal
    #[test]
    fn test_compare_lines_set3() {
        let file1_path = Path::new("src/testdata/set3/file1.txt");
        let file2_path = Path::new("src/testdata/set3/file2.txt");
        let file3_path = Path::new("src/testdata/set3/file3.txt");

        let file1 = fs::read_to_string(file1_path).unwrap();
        let file2 = fs::read_to_string(file2_path).unwrap();
        let file3 = fs::read_to_string(file3_path).unwrap();

        assert!(!compare_lines(vec![file1, file2, file3]));
    }

    // Test files with similar but different content are not equal
    #[test]
    fn test_compare_lines_set4() {
        let file1_path = Path::new("src/testdata/set4/file1.txt");
        let file2_path = Path::new("src/testdata/set4/file2.txt");

        let file1 = fs::read_to_string(file1_path).unwrap();
        let file2 = fs::read_to_string(file2_path).unwrap();

        assert!(!compare_lines(vec![file1, file2]));
    }
}
