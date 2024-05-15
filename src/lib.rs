pub fn find_match(content: &str, pattern: &str, mut writer: impl std::io::Write){
    for line in content.lines(){
        if line.contains(pattern)
        {
            match writeln!(writer, "{}", line) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to output: {}", e);
                    return;
                }
            }
        }
    }
}