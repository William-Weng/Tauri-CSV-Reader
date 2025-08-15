#[macro_export]
macro_rules! ww_print {
    ($message:expr) => {
        println!("\n[{file} - {line}]\n{message}",
            file = file!().green().bold(),
            line = format!("line.{}", line!().to_string()).yellow().bold(),
            message = $message
        )
    };
}