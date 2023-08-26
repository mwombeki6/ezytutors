use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path ="../handlers.rs"]
mod handlers;
#[path ="../routes.rs"]
mod routes;
#[path ="../state.rs"]
mod state;