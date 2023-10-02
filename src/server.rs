//! # server
//! A simple module for managing server easily.

use configuration;
use service;
use slog;
use weld;
use hyper::Server;
use futures_cpupool::CpuPool;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};


/// Holds server configuration and logger
pub struct WeldServer<'a> {
    //Configuration of the server for future access.
    configuration: &'a configuration::Server,
    //Logger for the server. All services should create loggers from this.
    logger: slog::Logger,
}

impl<'a> WeldServer<'a> {
    /// Creates a new instance of Server
    pub fn new(config: &'a configuration::Server) -> WeldServer<'a> {
        WeldServer {
            configuration: config,
            logger: weld::ROOT_LOGGER.new(o!()),
        }
    }

    /// Starts the server. **Server event loop will run on the same thread with the thread this function called. Be careful.**
    pub fn start(&self) {
        let endpoint =
            format!("{}:{}", self.configuration.host, self.configuration.port).parse().unwrap();
        info!(self.logger, "Server - Started ! {}", &endpoint);

        let thread_pool = CpuPool::new_num_cpus();



        
        async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
            Ok(Response::new(Body::from("Hello World!")))
        }

        let server = Server:bind(&endpoint).serve(make_svc);

        match Http::new()
                    .bind(&endpoint, move || {
                        Ok(service::RestService {
                            logger: weld::ROOT_LOGGER.new(o!()),
                            thread_pool: thread_pool.clone(),
                        })
                    })
        {

            Ok(_v) => {
                _v.run().unwrap();
            },
            Err(_e) => {
                println!("Oops, something went wrong!");
                println!("{}", _e);
                return;
            }
        }

    }  // end of start()

}  // end of Server
