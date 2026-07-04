use crate::bot::Bot;

pub async fn load_slash_commands(bot: &Bot) -> usize {
    let count = bot.slash_commands.len();
    if count == 0 {
        println!("  ⚠️ [Slash] No commands found");
    } else {
        println!("  ✅ [Slash] {} commands loaded", count);
    }
    count
}
