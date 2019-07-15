extern crate protobuf;
extern crate grpcio;
extern crate futures;

pub mod protos;
pub mod dbengine;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};
use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};


use protos::kvserver::{Request,Reponse,Operation};
use protos::kvserver_grpc::{self, Kvdb};

use dbengine::dbengine::DbEngine;

#[derive(Clone)]
struct DbService{
    db_engine: DbEngine,
}

impl Kvdb for DbService {
    fn db(&mut self, ctx: RpcContext, req: Request, sink: UnarySink<Reponse>) {
        let mut response = Reponse::new();
        println!("Recieve response{{ {:?} }}",req );
        let engine = &mut self.db_engine;
        let operation = &req.oper;
        match operation {
            Operation::get => {
                let ret = engine.get(&req.key);
                match ret {
                    Ok(value) => {
                        response.set_str1(String::from("Success!"));
                        response.set_str2(value);
                    }
                    Err(_) => {
                        response.set_str1(String::from("fail!"));
                        response.set_str2(String::from("Nothing to show."));
                    }
                }
            }
            Operation::set => {
                let ret = engine.set(&req.key,&req.value);
                match ret {
                    Ok(value) => {
                        response.set_str1(String::from("Success!"));
                        response.set_str2(value);
                    }
                    Err(_) => {
                        response.set_str1(String::from("New insert."));
                        response.set_str2(String::from("Nothing to show."));
                    }
                }
            }
            Operation::del => {
                let ret = engine.delete(&req.key);
                match ret {
                    Ok(value) => {
                        response.set_str1(String::from("Success!"));
                        response.set_str2(value);
                    }
                    Err(_) => {
                        response.set_str1(String::from("fail!"));
                        response.set_str2(String::from("Nothing to show."));
                    }
                }
            }
            Operation::scan => {
                let ret = engine.scan(&req.key, &req.value);
                let mut strings = String::from(" ");
                
                if(ret.database.is_empty()) {
                    response.set_str1(String::from("Not Found!"));
                    response.set_str2(String::from("Nothing to show"));
                }else{
                    for (key, value) in ret.database.iter() {
                        strings.insert_str(0,"\n");
                        strings.insert_str(0,value);
                        strings.insert_str(0,"--");
                        strings.insert_str(0,key);
                    }
                    response.set_str1(String::from("Found!"));
                    response.set_str2(strings);
                }
            }
        }

    let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
    ctx.spawn(f)    
    }
}

impl DbService{
    pub fn new() -> Self {
        println!("new Dbservice" );

        DbService {
            db_engine: DbEngine::new(),
        }
    }
}

fn main(){
    let env = Arc::new(Environment::new(1));
    let service  = kvserver_grpc::create_kvdb(DbService::new());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 20001)
        .build()
        .unwrap();

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
