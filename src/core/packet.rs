use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::models::File;

#[derive(Clone, Debug, Deserialize)]
pub enum InPacket {
    Message {
        parent_id: Option<i32>,
        content: String,
        media: Option<Vec<Option<File>>>,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct ActiveUser {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub pfp: Option<File>,
    pub banner: Option<File>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct InComment {
    pub user: ActiveUser,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub media: Option<Vec<Option<File>>>,
}

#[derive(Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct OutComment {
    pub user: ActiveUser,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub media: Option<Vec<Option<File>>>,
}

#[derive(Debug, Message, Serialize)]
#[rtype(result = "()")]
pub enum OutPacket {
    ConnectNotification(ConnectNotification),
    DisconnectNotification(DisconnectNotification),
    OutComment(OutComment),
}

#[derive(Clone, Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct ConnectNotification {
    pub user: ActiveUser,
}

#[derive(Clone, Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct DisconnectNotification {
    pub id: i32,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<OutPacket>,
    pub id: String,
    pub post_id: i32,
    pub notif: ConnectNotification,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
    pub post_id: i32,
    pub notif: DisconnectNotification,
}

#[derive(Debug, Message)]
#[rtype(result = "Vec<ActiveUser>")]
pub struct ListActiveUsers(i32);
