use std::collections::HashMap;

use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
use nidrs::macros::{controller, get, post};
use nidrs::{Inject, Meta};

use crate::AppResult;

use super::{dto::Status, service::AppService};

#[controller()]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[get("/hello")]
    pub async fn get_hello_world(
        &self,
        meta: Meta,
        Query(q): Query<HashMap<String, String>>,
    ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
        println!("Query {:?}", q);
        println!("Meta {:?}", meta.get::<&str>("role"));

        Ok((
            AppendHeaders([
                ("X-Custom-Header".to_string(), "hello".to_string()),
                ("X-Custom-Header".to_string(), "world".to_string()),
            ]),
            Status {
                db: "ok".to_string(),
                redis: "ok".to_string(),
            },
        ))
    }

    #[get("/hello2")]
    pub async fn get_hello_world2(
        &self,
        Query(q): Query<HashMap<String, String>>,
    ) -> AppResult<String> {
        println!("Query {:?}", q);

        shared::test_tools::fn_test()?;

        Ok(self.app_service.get_hello_world())
    }

    #[post("/hello")]
    pub async fn post_hello_world(
        &self,
        Query(q): Query<HashMap<String, String>>,
        Json(j): Json<serde_json::Value>,
    ) -> AppResult<String> {
        println!("Query {:?}", q);
        println!("Json {:?}", j);

        Ok("Hello, World2!".to_string())
    }
}
