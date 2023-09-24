use std::sync::{Arc};
use std::fmt::{Display, Formatter, Result};
use futures::lock::Mutex;
use clap::Parser;
use log::info;
use async_trait::async_trait;
use mediator::DefaultAsyncMediator;
use tokio::io::AsyncBufReadExt;

use mediator::{Request, AsyncMediator};

use super::HostedService;

pub struct ConsoleHostedService {
  mediator: Arc<Mutex<DefaultAsyncMediator>>,
  join_handle: Option<tokio::task::JoinHandle<()>>
}

impl ConsoleHostedService {
  pub fn new(mediator: DefaultAsyncMediator) -> Self {
    ConsoleHostedService {
      mediator: Arc::new(Mutex::new(mediator)),
      join_handle: None
    }    
  }
}

#[derive(clap::Parser)]
struct ConsoleCommand {
  #[command(subcommand)]
  command: Subcommand
}

#[derive(clap::Subcommand)]
enum Subcommand {
  GetRooms(GetRoomsRequestModel),
  AddRoom(AddRoomRequestModel),
}

#[derive(clap::Args)]
pub struct GetRoomsRequestModel {

}

pub struct RoomModel {

}

impl RoomModel {
  pub fn new() -> Self {
    RoomModel { }
  }
}

impl Display for RoomModel {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", "")?;
    Ok(())
  }
}

pub struct GetRoomsResponseModel {
  rooms: Vec<RoomModel>
}

impl GetRoomsResponseModel {
  pub fn new(rooms: Vec<RoomModel>) -> Self {
    GetRoomsResponseModel { rooms }
  }
}

impl Display for GetRoomsResponseModel {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for room in &self.rooms {
      write!(f, "{}", room)?;
    }

    Ok(())
  }
}

#[derive(clap::Args)]
pub struct AddRoomRequestModel {

}

impl Request<AddRoomResponseModel> for AddRoomRequestModel {

}

pub struct AddRoomResponseModel {
  room : RoomModel
}

impl AddRoomResponseModel {
  pub fn new(room: RoomModel) -> Self {
    AddRoomResponseModel { room }
  }
}

impl Display for AddRoomResponseModel {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.room)?;

    Ok(())
  }
}

impl Request<GetRoomsResponseModel> for GetRoomsRequestModel {

}

#[async_trait]
impl HostedService for ConsoleHostedService {  
  async fn start(&mut self) {
    info!("test hosted service start.");

    let stdin = tokio::io::stdin();
    let reader = tokio::io::BufReader::new(stdin);
    let mut lines = reader.lines();   

    let cloned_mediator = self.mediator.clone();

    self.join_handle = Some(tokio::spawn(async move {
      while let Some(line) = lines.next_line().await.unwrap() {
        println!("{:?} {}", std::thread::current().id(), line);

        let mut v = vec![""];
        v.extend(line.split_whitespace());

        let console_command = match ConsoleCommand::try_parse_from(v) {
          Err(error) => {
            println!("error: invalid {:?}", error);
            continue;
          },
          Ok(console_command) => console_command
        };

        let mut locked_mediator = cloned_mediator.lock().await;
        // let mut locked_mediator = cloned_mediator.lock().unwrap();

        match console_command.command {
          Subcommand::GetRooms(request) => print(locked_mediator.send(request).await.unwrap()),
          Subcommand::AddRoom(request) => print(locked_mediator.send(request).await.unwrap()),
        }
      }    
    }));
  }

  async fn stop(&mut self) {    
    info!("test hosted service stop.");
  }
}

fn print(display: impl Display) {
  println!("{}", display)
}