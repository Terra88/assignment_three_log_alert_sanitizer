fn extract_severity(log: &str) -> &str {
    if let (Some(start), Some(end)) = (log.find("["), log.find("]"))  {
        if start < end {
            return &log[start + 1..end];
        }
    }
    log
}

fn append_alert(summary: &mut String, severity: &str, message: &str) {
    summary.push_str(&format!("[{}] {}\n", severity, message));
}

fn archive_log(mut log: String, code: u32) -> String{
    log.push_str(&format!(" [CODE: {}]", code));
    log
}

fn main() {
    let raw_log = String::from("[WARN] Disk space reaching 90%");
    let status_code: u32 = 404;
    
    let log = extract_severity(&raw_log);
    println!("Extracted log: {}", log);
    
    let mut summary = String::new();
    append_alert(&mut summary, log, "Disk space low");
    
    println!("\nSystem Log Summary:");
    print!("{}", summary);
    
    let archived = archive_log(raw_log, status_code);
    println!("Archived output: {}", archived);
}