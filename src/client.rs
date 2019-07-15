extern crate protobuf;
extern crate grpcio;
extern crate futures;

pub mod protos;


use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::kvserver::{Request, Reponse, Operation};
use protos::kvserver_grpc::KvdbClient;


struct Client {
    client: KvdbClient,
}

impl Client {
    pub fn new(host: String, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(addr.as_ref());
        let kv_client = KvdbClient::new(ch);

        Client {
            client: kv_client,
        }
    }

    pub fn get (&self, key: String) {
        let mut requst = Request::new();
        requst.set_oper(Operation::get);
        requst.set_key(key);
        requst.set_value(String::from(" "));
        let ret = self.client.db(&requst).expect("RPC failed");
        println!("{}\n{}",ret.str1, ret.str2 );
    }

    pub fn set(&self, key: String, value: String) {
        let mut requst = Request::new();
        requst.set_oper(Operation::set);
        requst.set_key(key);
        requst.set_value(value);
        let ret = self.client.db(&requst).expect("RPC failed");
        println!("{}\n{}",ret.str1, ret.str2 );
    }

    pub fn delete(&self, key: String) {
        let mut requst = Request::new();
        requst.set_oper(Operation::del);
        requst.set_key(key);
        requst.set_value(String::from(" "));
        let ret = self.client.db(&requst).expect("RPC failed");
        println!("{}\n{}",ret.str1, ret.str2 );
    }

    pub fn scan(&self, key: String, value: String) {
        let mut requst = Request::new();
        requst.set_oper(Operation::scan);
        requst.set_key(key);
        requst.set_value(value);
        let ret = self.client.db(&requst).expect("RPC failed");
        println!("{}\n{}",ret.str1, ret.str2 );
    }
}


fn main() {
    let test_host = String::from("127.0.0.1");
    let test_port = 20001;

    let client = Client::new(test_host.clone(), test_port);
    client.set("aa".to_string(),"aaaaa".to_string());
    client.set("bb".to_string(),"bbbbb".to_string());
    client.set("cc".to_string(),"ccccc".to_string());
    client.get("aa".to_string());
    client.delete("aa".to_string());
    client.scan("aa".to_string(),"cc".to_string());
}