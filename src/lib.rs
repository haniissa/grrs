use std::io::BufRead;

pub fn find_matches(content: impl BufRead, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                let _ = writeln!(writer, "{}", line);
            }
        }
    }
}

