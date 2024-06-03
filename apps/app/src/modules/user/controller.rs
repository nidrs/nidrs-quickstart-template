use std::collections::HashMap;

use nidrs::externs::axum::extract::Query;
use nidrs::macros::{controller, get};
use nidrs::{AppResult, Inject};

use super::service::UserService;

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[get("/hello")]
    pub async fn get_hello_world(
        &self,
        Query(q): Query<HashMap<String, String>>,
    ) -> AppResult<String> {
        println!("Query {:?}", q);
        Ok(self.user_service.extract().get_hello_world2())
    }
}
