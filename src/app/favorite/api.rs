use super::{
    response,
    service::{self, UnfavoriteService},
};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

type ArticleIdSlug = String;

pub async fn favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid."); // TODO: validate

    // TODO: validate article_id

    let (article, profile, favorite_info, tags_list) = service::favorite(
        &conn,
        &service::FavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    )?;
    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tags_list));

    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfavorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid."); // TODO: validate

    // TODO: validate article_id

    let (article, profile, favorite_info, tags_list) = service::unfavorite(
        &conn,
        &UnfavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    )?;
    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tags_list));
    Ok(HttpResponse::Ok().json(res))
}
