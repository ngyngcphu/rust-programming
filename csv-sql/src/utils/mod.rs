pub fn get_data_directory() -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let data_dir = home_dir.join(".csv.sql");
    data_dir.to_string_lossy().into_owned()
}
