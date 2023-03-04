use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, Error};
use actix_web::{web, HttpResponse};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use crate::{actions, models};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_all_users(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/users/{id}")]
async fn get_user_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_user(&mut conn, id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = models::JsonMessage {
            message: format!("could not find user by id = {id}"),
        };
        Ok(HttpResponse::NotFound().json(res))
    }
}

#[post("/users")]
async fn post_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        actions::create_user(&mut conn, &form.name, &form.email)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let res = models::JsonMessage {
        message: "New user created!".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}

#[get("/messages")]
async fn get_messages(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_all_messages(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[post("/users/{id}/messages")]
async fn post_user_message(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    form: web::Json<models::NewMessage>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    web::block(move || {
        let mut conn = pool.get()?;
        actions::create_message(&mut conn, id, &form.msg)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let res = models::JsonMessage {
        message: "New message created".to_string(),
    };
    Ok(HttpResponse::Ok().json(res))
}
