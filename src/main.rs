use actix_web::{App,HttpServer,web};
use listenfd::ListenFd;
use actix_web::middleware::Logger;
// use env_logger;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod auth;
pub mod models;
pub mod schema;
pub mod controller;
pub mod connect;
pub mod router;
pub mod middleware;
use router::user_routes;
use env_logger::Env;

#[actix_rt::main]
async fn main()->std::io::Result<()>{
    let mut listenfd=ListenFd::from_env();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let mut  server=HttpServer::new(move||{
        App::new()
        
        // .wrap(Logger::default())
        // .wrap(Logger::new("%a %{User-Agent}i"))
      
            .configure(user_routes::scoped_config)
        // .service(web::scope("/user").configure(user_routes::scoped_config))
       
    });
    server=if let Some(l)=listenfd.take_tcp_listener(0)
    .unwrap(){
        server.listen(l)?
    }else{
        server.bind("127.0.0.1:3000")?
    };
    server.run().await

  
}