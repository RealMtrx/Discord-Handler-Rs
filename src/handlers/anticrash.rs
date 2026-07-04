use std::panic;

pub fn setup_anticrash() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        eprintln!("  ❌ [AntiCrash] Panic recovered: {} at {}", msg, location);

        if let Some(bt) = std::backtrace::Backtrace::capture() {
            eprintln!("  Stack trace:\n{:?}", bt);
        }
    }));

    println!("  ✅ [AntiCrash] Active");
}
