use async_trait::async_trait;
use log::info;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator};

use crate::hosting::{RoomModel, GetRoomsRequestModel, GetRoomsResponseModel, AddRoomRequestModel, AddRoomResponseModel};

pub struct ConsoleController {
}


impl ConsoleController {
    pub fn new(default_async_mediator: DefaultAsyncMediator) -> Self {
        Self {

        }
    }
}

#[async_trait]
impl AsyncRequestHandler<GetRoomsRequestModel, GetRoomsResponseModel> for ConsoleController {
    async fn handle(&mut self, request: GetRoomsRequestModel) -> GetRoomsResponseModel {
        info!("get rooms");
        GetRoomsResponseModel::new(vec![])
    }
}

#[async_trait]
impl AsyncRequestHandler<AddRoomRequestModel, AddRoomResponseModel> for ConsoleController {
    async fn handle(&mut self, request: AddRoomRequestModel) -> AddRoomResponseModel {
        info!("add room");
        AddRoomResponseModel::new(RoomModel::new())
    }
}