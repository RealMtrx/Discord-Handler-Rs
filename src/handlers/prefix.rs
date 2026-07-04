use crate::bot::Bot;

pub fn load_prefix_commands(bot: &Bot) -> usize {
    let count = bot.prefix_commands.len();
    if count == 0 {
        println!("  ⚠️ [Prefix] No commands found");
    } else {
        println!("  ✅ [Prefix] {} commands loaded", count);
    }
    count
}
