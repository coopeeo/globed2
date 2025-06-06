use std::net::IpAddr;

use globed_shared::{
    GameServerBootData, MAX_SUPPORTED_PROTOCOL, SERVER_MAGIC,
    esp::{ByteBuffer, ByteBufferExtWrite, types::FastString},
    logger::debug,
};

use rocket::{State, post};

use blake2::{Blake2b, Digest};
use digest::consts::U32;

use crate::{config::UserlistMode, state::ServerState, web::*};

#[post("/gs/boot")]
pub async fn boot(
    state: &State<ServerState>,
    password: GameServerPasswordGuard,
    ip_address: IpAddr,
    user_agent: GameServerUserAgentGuard<'_>,
) -> WebResult<Vec<u8>> {
    let correct = state.state_read().await.config.game_server_password.clone();

    if !password.verify(&correct) {
        unauthorized!("invalid gameserver credentials");
    }

    let state = state.state_read().await;
    let config = &state.config;

    let motd_hash = if state.motd.is_empty() {
        String::new()
    } else {
        let mut hasher = Blake2b::<U32>::new();
        hasher.update(state.motd.clone().as_bytes());
        let output = hasher.finalize();

        let mut motd_hash = String::with_capacity(output.len() * 2);
        for byte in output.iter() {
            motd_hash.push_str(&format!("{:02x}", byte));
        }

        motd_hash
    };

    let bdata = GameServerBootData {
        protocol: MAX_SUPPORTED_PROTOCOL,
        tps: config.tps,
        maintenance: config.maintenance,
        secret_key2: config.secret_key2.clone(),
        token_expiry: config.token_expiry,
        status_print_interval: config.status_print_interval,
        admin_key: FastString::new(&config.admin_key),
        whitelist: config.userlist_mode == UserlistMode::Whitelist,
        admin_webhook_url: config.admin_webhook_url.clone(),
        rate_suggestion_webhook_url: config.rate_suggestion_webhook_url.clone(),
        featured_webhook_url: config.featured_webhook_url.clone(),
        room_webhook_url: config.room_webhook_url.clone(),
        chat_burst_limit: config.chat_burst_limit,
        chat_burst_interval: config.chat_burst_interval,
        roles: config.roles.clone(),
        motd: state.motd.clone(),
        motd_hash,
        motd_dynamic: state.motd_dynamic,
    };

    debug!("boot data request from game server {} at {}", user_agent.0, ip_address);

    let mut bb = ByteBuffer::new();
    bb.write_bytes(SERVER_MAGIC);
    bb.write_value(&bdata);

    drop(state);
    Ok(bb.into_vec())
}
