use super::presenters::TagPresenter;
use super::repositories::TagRepository;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct TagUsecase {
    tag_repository: Arc<dyn TagRepository>,
    tag_presenter: Arc<dyn TagPresenter>,
}

impl TagUsecase {
    pub fn new(
        tag_repository: Arc<dyn TagRepository>,
        tag_presenter: Arc<dyn TagPresenter>,
    ) -> Self {
        Self {
            tag_repository,
            tag_presenter,
        }
    }

    pub fn list(&self) -> Result<HttpResponse, AppError> {
        let list = self.tag_repository.list()?;
        let res = self.tag_presenter.to_json(list);
        Ok(res)
    }
}