use async_trait::async_trait;

use crate::domains::Room;

#[async_trait]
pub trait RoomRepository {
    async fn get_rooms() -> Vec<Room>;
}