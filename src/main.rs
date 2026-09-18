//1. function borrows original string (&str) and returns sliced (&str)
fn extract_severity(log: &str) -> &str {
    //searches bytes(and indexes) of brackets '[' and ']'
    if let (Some(start), Some(end)) = (log.find("["), log.find("]"))  {
        //Ensure, that start- and end points are in right order
        if start < end {
            //return sliced text between the brackets and leave out the brackets.
            return &log[start + 1..end];
        }
    }
    //if brackets are not found, return the original string
    log
}

//2. Add alert summary
//takes in variable reference(&mut String) summary,
//so that it can be edited in-place.
fn append_alert(summary: &mut String, severity: &str, message: &str) {
    //format how summary is presented and append it to the end of the summary buffer.
    summary.push_str(&format!("[{}] {}\n", severity, message));
}

//3. Archive log entry
//Takes full ownership of the log-string and code-state.
fn archive_log(mut log: String, code: u32) -> String{
    //Edits the owned String buffer by appending the status code
    log.push_str(&format!(" [CODE: {}]", code));
    //Transfers ownership of the updated String back to the log
    log
}

fn main() {
    //4. main:
    //allocate an owned string on the heap (raw_log) and an integer on the stack
    let raw_log = String::from("[WARN] Disk space reaching 90%");
    let status_code: u32 = 404;

    // Borrow raw_log immutably
    // 'log' is a slice referencing the memory within raw_log.
    let log = extract_severity(&raw_log);
    println!("Extracted log: {}", log);

    // Allocate an empty string and pass a mutable reference(&mut summary)
    let mut summary = String::new();
    append_alert(&mut summary, log, "Disk space low");

    println!("\nSystem Log Summary:");
    print!("{}", summary);

    // Move full ownership of raw_log into archive_log.
    // raw_log can no longer be accessed in main after this call.
    let archived = archive_log(raw_log, status_code);
    println!("Archived output: {}", archived);
}
