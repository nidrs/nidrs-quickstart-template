use nidrs::default_uses;
use nidrs::macros::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

#[module({
    imports: [
        UserModule,
    ],
    // interceptors: [LogInterceptor],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
