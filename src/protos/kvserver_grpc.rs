// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KVDB_DB: ::grpcio::Method<super::kvserver::Request, super::kvserver::Reponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kvserver.Kvdb/db",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KvdbClient {
    client: ::grpcio::Client,
}

impl KvdbClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvdbClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn db_opt(&self, req: &super::kvserver::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvserver::Reponse> {
        self.client.unary_call(&METHOD_KVDB_DB, req, opt)
    }

    pub fn db(&self, req: &super::kvserver::Request) -> ::grpcio::Result<super::kvserver::Reponse> {
        self.db_opt(req, ::grpcio::CallOption::default())
    }

    pub fn db_async_opt(&self, req: &super::kvserver::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvserver::Reponse>> {
        self.client.unary_call_async(&METHOD_KVDB_DB, req, opt)
    }

    pub fn db_async(&self, req: &super::kvserver::Request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvserver::Reponse>> {
        self.db_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Kvdb {
    fn db(&mut self, ctx: ::grpcio::RpcContext, req: super::kvserver::Request, sink: ::grpcio::UnarySink<super::kvserver::Reponse>);
}

pub fn create_kvdb<S: Kvdb + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KVDB_DB, move |ctx, req, resp| {
        instance.db(ctx, req, resp)
    });
    builder.build()
}
