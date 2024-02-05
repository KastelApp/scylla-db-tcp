use chrono::DateTime;
use futures_util::{stream::SplitSink, SinkExt};
use scylla::frame::value::CqlTimestamp;
use serde_json::Result;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    calculate_hash::calculate_hash,
    state,
    structs::{
        common::{Command, CommandData, QueryResult},
        custom::{
            dms::Dms, friends::Friends, guild_members::{GuildMembers, GuildMembersCQL}, messages::Messages, roles::{Roles, RolesCQL}, settings::{CqlSettings, Settings}, udt::{
                BigintPair, BigintPairUDT, GuildOrderType, GuildOrderTypeUDT, MemberTimeouts, MemberTimeoutsUDT, MentionsType, MentionsTypeUDT, TokenType, TokensTypeUDT
            }
        },
        insert::InsertResponse,
    },
    util::queries::insert_query,
    LOGGING,
};

pub async fn insert(
    write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    command: &CommandData,
    user: Arc<Mutex<state::ClientState>>,
    keyspace: &Option<String>,
    table: &Option<String>,
    raw_command: &Command,
) {
    let user = user.lock().await;

    if !user.connected {
        let error = Command {
            command: "error".to_string(),
            data: CommandData::SelectResponse(QueryResult {
                error: Some("Not connected to Scylla".to_string()),
                result: Vec::new(),
            }),
            keyspace: None,
            table: None,
            hash: "".to_string(),
            length: "".len(),
            nonce: None,
            type_: None,
        };

        let response = serde_json::to_string(&error).unwrap();

        let mut write = write.lock().await;

        match write.send(Message::Text(response)).await {
            // ? we don't care about if it succeeds or not
            _ => {}
        }

        match write.close().await {
            _ => {}
        }

        return;
    }

    match command {
        CommandData::Insert(insert_data) => {
            let session = user.session.as_ref().unwrap().lock().await;

            let table = table.as_ref().unwrap();
            let keyspace = keyspace.as_ref();
            let user_keyspace = &user.keyspace;

            let query = insert_query(
                &keyspace.to_owned().unwrap_or(user_keyspace),
                table,
                insert_data,
            );

            if raw_command.type_.is_some() {
                let json: String = serde_json::to_string(&insert_data.columns).unwrap();

                match raw_command.type_.as_deref() {
                    Some("settings") => {
                        let result: Result<Settings> = serde_json::from_str(&json);

                        match result {
                            Ok(settings) => {
                                match settings {
                                    Settings {
                                        user_id,
                                        status,
                                        custom_status,
                                        bio,
                                        tokens,
                                        theme,
                                        language,
                                        privacy,
                                        mentions,
                                        max_guilds,
                                        max_file_upload_size,
                                        guild_order,
                                        allowed_invites,
                                    } => {
                                        let tokens = tokens
                                            .into_iter()
                                            .map(|token| {
                                                let TokenType {
                                                    created_date,
                                                    flags,
                                                    ip,
                                                    token_id,
                                                    token_,
                                                } = token;
                                                TokensTypeUDT {
                                                    flags,
                                                    ip,
                                                    token_id,
                                                    token_,
                                                    created_date: CqlTimestamp(
                                                        DateTime::parse_from_rfc3339(&created_date)
                                                            .unwrap()
                                                            .timestamp_millis(),
                                                    ),
                                                }
                                            })
                                            .collect::<Vec<TokensTypeUDT>>();

                                        let mentions = mentions
                                            .into_iter()
                                            .map(|mention| {
                                                let MentionsType { message_id } = mention;
                                                MentionsTypeUDT { message_id }
                                            })
                                            .collect::<Vec<MentionsTypeUDT>>();

                                        let guild_order = guild_order
                                            .into_iter()
                                            .map(|order| {
                                                let GuildOrderType { guild_id, position } = order;
                                                GuildOrderTypeUDT { guild_id, position }
                                            })
                                            .collect::<Vec<GuildOrderTypeUDT>>();

                                        let setting_type = CqlSettings {
                                            allowed_invites,
                                            bio,
                                            custom_status,
                                            guild_order,
                                            language,
                                            max_file_upload_size,
                                            max_guilds,
                                            mentions,
                                            privacy,
                                            status,
                                            theme,
                                            tokens,
                                            user_id,
                                        };

                                        match session.query(query.query, setting_type).await {
                                            Ok(_) => {
                                                let mut response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: None,
                                                            success: true,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: raw_command.nonce.clone(), // todo: do not clone
                                                    type_: None,
                                                };

                                                let string_response =
                                                    serde_json::to_string(&response.data).unwrap();

                                                response.length =
                                                    string_response.len() + response.command.len();

                                                response.hash = calculate_hash(
                                                    response.command.to_string()
                                                        + &response.length.to_string()
                                                        + &string_response,
                                                );

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                            Err(error) => {
                                                print!("Error: {}", error);

                                                let response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: Some(error.to_string()),
                                                            success: false,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: None,
                                                    type_: None,
                                                };

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error :(: {}", e);
                            }
                        }
                    }
                    Some("dm") => {
                        let result: Result<Dms> = serde_json::from_str(&json);
                    }
                    Some("friends") => {
                        let result: Result<Friends> = serde_json::from_str(&json);
                    }
                    Some("guild_members") => {
                        let result: Result<GuildMembers> = serde_json::from_str(&json);

                        match result {
                            Ok(guild_member) => {
                                match guild_member {
                                    GuildMembers {
                                        flags,
                                        guild_id,
                                        guild_member_id,
                                        joined_at,
                                        nickname,
                                        user_id,
                                        timeouts,
                                        roles
                                    } => {
                                        let timeouts = timeouts
                                            .into_iter()
                                            .map(|timeout| {
                                                let MemberTimeouts {
                                                    channel_id,
                                                    timeout_until
                                                } = timeout;

                                                MemberTimeoutsUDT {
                                                    channel_id,
                                                    timeout_until: CqlTimestamp(
                                                        DateTime::parse_from_rfc3339(&timeout_until)
                                                            .unwrap()
                                                            .timestamp_millis(),
                                                    ),
                                                }
                                            })
                                            .collect::<Vec<MemberTimeoutsUDT>>();

                                        let guild_member_type = GuildMembersCQL {
                                            flags,
                                            guild_id,
                                            guild_member_id,
                                            joined_at: CqlTimestamp(
                                                DateTime::parse_from_rfc3339(&joined_at)
                                                    .unwrap()
                                                    .timestamp_millis(),
                                            ),
                                            nickname,
                                            roles,
                                            timeouts,
                                            user_id
                                        };

                                        match session.query(query.query, guild_member_type).await {
                                            Ok(_) => {
                                                let mut response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: None,
                                                            success: true,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: raw_command.nonce.clone(), // todo: do not clone
                                                    type_: None,
                                                };

                                                let string_response =
                                                    serde_json::to_string(&response.data).unwrap();

                                                response.length =
                                                    string_response.len() + response.command.len();

                                                response.hash = calculate_hash(
                                                    response.command.to_string()
                                                        + &response.length.to_string()
                                                        + &string_response,
                                                );

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                            Err(error) => {
                                                print!("Error: {}", error);

                                                let response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: Some(error.to_string()),
                                                            success: false,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: None,
                                                    type_: None,
                                                };

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error :(: {}", e);
                            }
                        }
                    }
                    Some("messages") => {
                        let result: Result<Messages> = serde_json::from_str(&json);
                    }

                    Some("roles") => {
                        let result: Result<Roles> = serde_json::from_str(&json);

                        match result {
                            Ok(role) => {
                                match role {
                                    Roles {
                                        allowed_age_restricted,
                                        color,
                                        deleteable,
                                        guild_id,
                                        hoisted,
                                        mentionable,
                                        role_id,
                                        position,
                                        name,
                                        permissions
                                    } => {
                                        let permissions = permissions
                                            .into_iter()
                                            .map(|permission| {
                                                let BigintPair { first, second } = permission;
                                                BigintPairUDT { first, second }
                                            })
                                            .collect::<Vec<BigintPairUDT>>();

                                        let role_type = RolesCQL {
                                            allowed_age_restricted,
                                            color,
                                            deleteable,
                                            guild_id,
                                            hoisted,
                                            mentionable,
                                            name,
                                            permissions,
                                            position,
                                            role_id
                                        };

                                        match session.query(query.query, role_type).await {
                                            Ok(_) => {
                                                let mut response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: None,
                                                            success: true,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: raw_command.nonce.clone(), // todo: do not clone
                                                    type_: None,
                                                };

                                                let string_response =
                                                    serde_json::to_string(&response.data).unwrap();

                                                response.length =
                                                    string_response.len() + response.command.len();

                                                response.hash = calculate_hash(
                                                    response.command.to_string()
                                                        + &response.length.to_string()
                                                        + &string_response,
                                                );

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                            Err(error) => {
                                                print!("Error: {}", error);

                                                let response = Command {
                                                    command: "insert".to_string(),
                                                    data: CommandData::InsertResponse(
                                                        InsertResponse {
                                                            error: Some(error.to_string()),
                                                            success: false,
                                                        },
                                                    ),
                                                    keyspace: None,
                                                    table: None,
                                                    hash: "".to_string(),
                                                    length: "".len(),
                                                    nonce: None,
                                                    type_: None,
                                                };

                                                let response =
                                                    serde_json::to_string(&response).unwrap();

                                                let mut write = write.lock().await;

                                                match write.send(Message::Text(response)).await {
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error :(: {}", e);
                            }
                        }
                    }

                    _ => {
                        println!("Invalid type: {:?}", raw_command.type_)
                    }
                }
            } else {
                match session.query(query.query, query.values).await {
                    Ok(_) => {
                        println!("Insert successful");
                        let mut response = Command {
                            command: "insert".to_string(),
                            data: CommandData::InsertResponse(InsertResponse {
                                error: None,
                                success: true,
                            }),
                            keyspace: None,
                            table: None,
                            hash: "".to_string(),
                            length: "".len(),
                            nonce: raw_command.nonce.clone(), // todo: do not clone
                            type_: None,
                        };

                        let string_response = serde_json::to_string(&response.data).unwrap();

                        response.length = string_response.len() + response.command.len();

                        response.hash = calculate_hash(
                            response.command.to_string()
                                + &response.length.to_string()
                                + &string_response,
                        );

                        let response = serde_json::to_string(&response).unwrap();

                        let mut write = write.lock().await;

                        match write.send(Message::Text(response)).await {
                            _ => {}
                        }
                    }
                    Err(error) => {
                        print!("Error: {}", error);

                        let response = Command {
                            command: "insert".to_string(),
                            data: CommandData::InsertResponse(InsertResponse {
                                error: Some(error.to_string()),
                                success: false,
                            }),
                            keyspace: None,
                            table: None,
                            hash: "".to_string(),
                            length: "".len(),
                            nonce: None,
                            type_: None,
                        };

                        let response = serde_json::to_string(&response).unwrap();

                        let mut write = write.lock().await;

                        match write.send(Message::Text(response)).await {
                            _ => {}
                        }
                    }
                }
            }
        }

        _ => {
            if *LOGGING.lock().await {
                println!("[Warn] A User sent an invalid command: {:?}", command);
            }
        }
    }
}
