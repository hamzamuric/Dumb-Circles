use ws::{listen, Handler, Message, Request, Response, Result, Sender};

static INDEX_HTML: &'static [u8] = include_bytes!("everything.html");

struct Server {
    out: Sender,
}

impl Handler for Server {

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),

            "/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg);
        self.out.broadcast(msg)
    }
}

fn main() {
    listen("0.0.0.0:8000", |out| Server { out }).unwrap()
}