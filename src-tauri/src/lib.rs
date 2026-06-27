// Nexus Core — Rust/Tauri v2 Backend
//
// Module declarations. The run() entry point is in app.rs.
// main.rs calls nexus_core_lib::app::run().

pub mod app;
pub mod backup;
pub mod config;
pub mod connection;
pub mod core;
pub mod core_installer;
pub mod diagnostics;
pub mod dispatcher;
pub mod dns;
pub mod engine;
pub mod event;
pub mod geo;
pub mod ipc;
pub mod logging;
pub mod migration;
pub mod models;
pub mod monitoring;
pub mod network;
pub mod node;
pub mod performance;
pub mod pipeline;
pub mod platform;
pub mod profile;
pub mod protocol;
pub mod proxy;
pub mod recovery;
pub mod release;
pub mod repository;
pub mod rule;
pub mod rule_engine;
pub mod ruleset;
pub mod runtime;
pub mod security;
pub mod service;
pub mod session;
pub mod statistics;
pub mod storage;
pub mod subscription;
pub mod telemetry;
pub mod transport;
pub mod tray;
pub mod tun;
pub mod tunnel;
pub mod utils;

// Hand-written prost types matching proto/geosite.proto.
// Avoids a build-time dependency on protoc while retaining
// the same wire format as v2fly domain-list-community.
pub mod proto {
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Domain {
        #[prost(string, tag = "1")]
        pub r#type: String,
        #[prost(string, tag = "2")]
        pub value: String,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GeoSite {
        #[prost(string, tag = "1")]
        pub tag: String,
        #[prost(message, repeated, tag = "2")]
        pub domain: Vec<Domain>,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct GeoSiteList {
        #[prost(message, repeated, tag = "1")]
        pub entry: Vec<GeoSite>,
    }
}
