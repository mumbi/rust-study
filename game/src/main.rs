mod hosting;
mod presentations;
mod infrastructures;
mod domains;

use hosting::AddRoomRequestModel;
use hosting::AddRoomResponseModel;
use hosting::GetRoomsRequestModel;
use hosting::GetRoomsResponseModel;
use mediator::{DefaultAsyncMediator, AsyncRequestHandler};
use anyhow::Result;
use tokio::signal::unix::{signal, SignalKind};

use hosting::HostedService;
use hosting::ConsoleHostedService;

use presentations::console::ConsoleController;
use infrastructures::RoomRepository;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger("game/config/log4rs.yaml")?;    
    let mediator = init_mediator();
    let mut hosted_services = init_hosted_services(mediator);

    for hosted_service in hosted_services.iter_mut() {
        hosted_service.start().await;
    }

    let mut sigint = signal(SignalKind::interrupt())?;
    sigint.recv().await;

    for hosted_service in hosted_services.iter_mut().rev() {
        hosted_service.stop().await;
    }

    Ok(())
}

fn init_logger(path : &str) -> Result<()> {
    log4rs::init_file(path, Default::default())
}

fn init_mediator() -> DefaultAsyncMediator {

    let room_repository = RoomRepository::new();

    let mediator = DefaultAsyncMediator::builder()
    .add_handler_deferred(get_rooms_handler)
    .add_handler_deferred(add_rooms_handler)
    .build();

    mediator
}

fn get_rooms_handler(default_async_mediator: DefaultAsyncMediator) -> impl AsyncRequestHandler<GetRoomsRequestModel, GetRoomsResponseModel> {
    ConsoleController::new(default_async_mediator)
}

fn add_rooms_handler(default_async_mediator: DefaultAsyncMediator) -> impl AsyncRequestHandler<AddRoomRequestModel, AddRoomResponseModel> {
    ConsoleController::new(default_async_mediator)
}

fn init_hosted_services(mediator: DefaultAsyncMediator) -> Vec<Box<dyn HostedService>> {
    let mut hosted_services : Vec<Box<dyn HostedService>> = Vec::new();
    hosted_services.push(Box::new(ConsoleHostedService::new(mediator)));

    hosted_services
}
// mod presentations;

// use std::io::{self, Write};
// use std::thread;
// use std::time::Duration;
// use tokio::runtime::Runtime;

// // console crates.
// use console::{style, Term};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // do_stuff().unwrap();

//     // presentations::consoles::console_controller::ConsoleController;

//     println!("1 {:?}", thread::current().id());

//     let rt = Runtime::new()?;

//     rt.block_on(async {
//         println!("2 {:?}", thread::current().id());

//         for i in 0..100 {
//             let result =tokio::spawn(async {
//                 println!("3 {:?}", thread::current().id());
    
//             }); //.await;
//         }

//         println!("4 {:?}", thread::current().id());
//     });

//     Ok(())
// }

// fn do_stuff() -> io::Result<()> {
//     let term = Term::stdout();
//     term.clear_last_lines(1)?;
//     term.write_line("Done counting!")?;
//     writeln!(&term, "Hello World!")?;

//     loop {
//         write!(&term, "chat: ")?;
        
//         let res = term.read_line()?;
//         term.clear_last_lines(1)?;
//         writeln!(&term, "{}", res)?;
//     }

//     // term.move_cursor_up(1)?;
//     // term.clear_line()?;

//     Ok(())
// }
