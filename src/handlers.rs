use actix_web::Responder;

pub async fn get_users() -> impl Responder {
    format!("Hello form get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("Hello from get user by id")
}

pub async fn add_user() -> impl Responder {
    format!("Hello form add user")
}

pub async fn delete_user() -> impl Responder {
    format!("Hello from delete user")
}