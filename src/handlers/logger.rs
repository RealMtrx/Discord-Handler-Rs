use crate::config::Config;

pub struct StartupData {
    pub name: String,
    pub slash_count: usize,
    pub prefix_count: usize,
    pub events_count: usize,
    pub models_count: usize,
    pub mongo_status: bool,
}

pub fn startup_report(data: &StartupData) {
    println!();
    println!(
        "╔══════════════════════════════════╗\n\
         ║     {:<30}║\n\
         ╚══════════════════════════════════╝",
        data.name
    );
    println!();

    let lines = vec![
        (format!("Slash Commands: {}", data.slash_count), true),
        (format!("Prefix Commands: {}", data.prefix_count), true),
        (format!("Events Loaded: {}", data.events_count), true),
        (format!("Models Loaded: {}", data.models_count), true),
        ("AntiCrash: Active".to_string(), true),
        (
            format!("MongoDB: Connected = {}", data.mongo_status),
            data.mongo_status,
        ),
    ];

    for (label, ok) in lines {
        if ok {
            println!("  ✅ {}", label);
        } else {
            println!("  ❌ {}", label);
        }
    }

    println!();
    println!(
        "[ {} ] {} is now online and fully operational.",
        chrono::Local::now().format("%d/%m/%Y %H:%M:%S"),
        data.name
    );
}
