use crate::models::articles::{ArticleNumWithTag, ArticleSummary, QuerySlice};
use crate::models::tag::{DeleteTag, TagsData};
use crate::models::token::TokenExtension;
use crate::models::user::UserType;
use crate::util::errors::ErrorCode;
use crate::{AppState, NewTag, TagCount, Tags};
use actix_web::web::{Data, Path, Query};
use actix_web::{web, web::Form, Error, HttpRequest, HttpResponse};
use uuid::Uuid;

pub struct TagApi;

impl TagApi {
    pub async fn create_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<NewTag>,
    ) -> Result<HttpResponse, Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = params.into_inner().insert(state.get_ref());
        match res {
            true => api_resp_ok!(),
            false => api_resp_err!("create_tag failed!"),
        }
    }

    pub async fn delete_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<DeleteTag>,
    ) -> Result<HttpResponse, Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let conn = state.db.connection();
        let res = Tags::delete_tag(&conn, params.into_inner().id);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("delete_tag failed!"),
        }
    }

    pub async fn get_tag_with_count(
        state: Data<AppState>,
        _req: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        let res = TagCount::view_tag_count(&state.db.connection());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("view_tag_with_count failed!"),
        }
    }

    pub async fn get_tag_without_count(
        state: Data<AppState>,
        _req: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        let res = Tags::view_list_tag(state.get_ref());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("get all tag failed!"),
        }
    }

    pub async fn edit_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<Tags>,
    ) -> Result<HttpResponse, Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let conn = state.db.connection();
        let res = params.into_inner().edit_tag(&conn);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("edit_tag failed!"),
        }
    }

    pub async fn update_tags(
        state: Data<AppState>,
        req: HttpRequest,
        mut body: web::Payload,
    ) -> Result<HttpResponse, Error> {
        debug!("update_tags");
        let res = extract_data!(body, TagsData);
        match res {
            Ok(data) => match data.execute(req, &state).await {
                Ok(_) => api_resp_ok!(),
                Err(e) => api_resp_err_with_code!(e.code, e.detail),
            },
            Err(e) => api_resp_err_with_code!(e.code, e.detail),
        }
    }

    async fn list_all_article_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        tag_id: Path<Uuid>,
        params: Query<QuerySlice>,
    ) -> Result<HttpResponse, Error> {
        info!("list_all_article_filter_by_tag");
        let token_ext = TokenExtension::from_request(&req);
        let is_admin = match token_ext {
            Some(t) => t.user_type == UserType::Admin,
            None => false,
        };
        let conn = &state.get_ref().db.connection();
        match ArticleSummary::list_articles_with_tag(
            conn,
            tag_id.into_inner(),
            params.limit,
            params.offset,
            is_admin,
        ) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    async fn get_article_number_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<ArticleNumWithTag>,
    ) -> Result<HttpResponse, Error> {
        let token_ext = TokenExtension::from_request(&req);
        let is_admin = match token_ext {
            Some(t) => t.user_type == UserType::Admin,
            None => false,
        };
        let count = ArticleSummary::query_article_numbers_with_tag(&state, params.tag_id, is_admin);
        match count {
            Ok(n) => {
                debug!("article count: {:?}", n);
                api_resp_data!(n)
            }
            Err(e) => api_resp_err!(format!("{:?}", e)),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/tag")
                .route(web::post().to(Self::create_tag)) // create
                .route(web::get().to(Self::get_tag_without_count)) // get article
                .route(web::delete().to(Self::delete_tag)) // delete
                .route(web::patch().to(Self::edit_tag)),
        )
        .service(
            web::resource("/tag/{tag_id}/articles")
                .route(web::get().to(Self::list_all_article_filter_by_tag)),
        )
        .service(
            web::resource("/tag/{tag_id}/articles/count")
                .route(web::get().to(Self::get_article_number_filter_by_tag)),
        )
        .service(
            web::resource("/tags").route(web::put().to(Self::update_tags)), // update
        )
        .service(
            web::resource("/tags/articles/count").route(web::get().to(Self::get_tag_with_count)),
        );
    }
}
