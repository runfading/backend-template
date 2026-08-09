use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub const DEFAULT_PAGE: u64 = 1;
pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MIN_PAGE_SIZE: u64 = 1;
pub const MAX_PAGE_SIZE: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, IntoParams, ToSchema)]
#[serde(default)] // 声明反序列默认值
#[into_params(parameter_in = Query)] // 必须要声明 = Query，否则生成的swagger展示不出来这两个参数
pub struct PageQuery {
    // #[serde(default = "default_page")] // 声明反序列化的默认值,没有这个请求无page会发生错误
    #[param(default = 1)] // 声明文档page默认值1,不影响反序列化值
    pub page: u64,

    // #[serde(default = "default_page_size")]
    #[param(default = 10)]
    pub page_size: u64,
}

impl Default for PageQuery {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

// fn default_page() -> u64 {
//     DEFAULT_PAGE
// }
//
// fn default_page_size() -> u64 {
//     DEFAULT_PAGE_SIZE
// }

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
