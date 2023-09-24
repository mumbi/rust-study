use async_trait::async_trait;

use crate::domains::{Room, repositories};

pub struct RoomRepository {

}

impl RoomRepository {
    pub fn new() -> Self {
        RoomRepository {

        }
    }
}

#[async_trait]
impl repositories::RoomRepository for RoomRepository {
    async fn get_rooms() -> Vec<Room> {
        vec![]
    }
}