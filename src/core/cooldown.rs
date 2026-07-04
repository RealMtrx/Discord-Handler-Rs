use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct CooldownManager {
    cooldowns: Arc<Mutex<HashMap<String, HashMap<String, tokio::time::Instant>>>>,
}

impl CooldownManager {
    pub fn new() -> Self {
        Self {
            cooldowns: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn check(
        &self,
        user_id: &str,
        command: &str,
        cooldown_ms: u64,
    ) -> (bool, u64) {
        let mut map = self.cooldowns.lock().await;

        let user_map = map.entry(command.to_string()).or_insert_with(HashMap::new);

        if let Some(expires) = user_map.get(user_id) {
            let now = tokio::time::Instant::now();
            if now < *expires {
                let remaining = expires.duration_since(now).as_secs();
                return (true, remaining);
            }
        }

        let expires = tokio::time::Instant::now() + Duration::from_millis(cooldown_ms);
        user_map.insert(user_id.to_string(), expires);

        let map_clone = self.cooldowns.clone();
        let uid = user_id.to_string();
        let cmd = command.to_string();
        tokio::spawn(async move {
            sleep(Duration::from_millis(cooldown_ms)).await;
            let mut m = map_clone.lock().await;
            if let Some(um) = m.get_mut(&cmd) {
                um.remove(&uid);
            }
        });

        (false, 0)
    }
}
