pub fn read_from_file(is_real: bool, day: u32, order: Option<u32>) -> Vec<String> {
    let suffix = match order {
        Some(o) => format!("_{}", o),
        None => "".to_string()
    };

    let data_file = if is_real {
        format!("real_data{}.txt", suffix)
    } else {
        format!("test_data{}.txt", suffix)
    };
    
    let source_folder = std::env::current_dir().expect("Failed to get current directory").join("src");
    let date_folder = format!("dec_{:02}", day);

    let data_file = source_folder.join(date_folder).join(data_file);
    let input = std::fs::read_to_string(data_file).expect("Failed to read file");
    
    input.lines().map(|s| s.to_string()).collect()
}
