pub fn format_code(source: Vec<u16>) -> String {
    if source.is_empty() {
        return String::new();
    }
    
    let mut formatted = String::with_capacity(source.len() * "0x0000: 0x0000\n".len());
    for (i, item) in source.iter().enumerate() {
        formatted.push_str(&format!("{:#06x}: {:#06x}\n", i, item));
    }
    formatted
}