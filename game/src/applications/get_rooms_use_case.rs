

struct Request {

}

impl Request<Response> for Request {

}

struct Response {

}

struct Handler {
    

}

impl AsyncRequestHandler for Handler {
    async fn handle(&mut self, request: Request) -> Response {
        info!("handle");
        
        Response {
            
        }
    }
}

