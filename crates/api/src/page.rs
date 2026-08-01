use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub const DEFAULT_PAGE: u64 = 1;
pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MIN_PAGE_SIZE: u64 = 1;
pub const MAX_PAGE_SIZE: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, IntoParams, ToSchema)]
pub struct PageQuery {
    #[serde(default = "default_page")]
    #[param(default = 1)]
    pub page: u64,
    #[serde(default = "default_page_size")]
    #[param(default = 10)]
    pub page_size: u64,
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

impl PageQuery {
    pub fn new(page: u64, page_size: u64) -> Self {
        Self { page, page_size }
    }

    pub fn normalized(&self) -> Self {
        Self {
            page: self.page.max(DEFAULT_PAGE),
            page_size: self.page_size.clamp(MIN_PAGE_SIZE, MAX_PAGE_SIZE),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

impl<T> PageResult<T> {
    pub fn new(items: Vec<T>, total: u64, page: u64, page_size: u64) -> Self {
        Self {
            items,
            total,
            page,
            page_size,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> PageResult<U> {
        PageResult {
            items: self.items.into_iter().map(f).collect(),
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        }
    }
}
