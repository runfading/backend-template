use crate::biz::demo::models::{CreateDemo, Demo, UpdateDemo};
use crate::error::DomainError;

pub trait DemoRepository {
    /// 查询
    fn list(&self) -> impl Future<Output = Result<Vec<Demo>, DomainError>> + Send;

    /// 根据id获取数据
    fn find_by_id(&self, id: i64)
    -> impl Future<Output = Result<Option<Demo>, DomainError>> + Send;

    /// 保存
    fn save(&self, demo: CreateDemo) -> impl Future<Output = Result<Demo, DomainError>> + Send;

    // 更新
    fn update(&self, patch: UpdateDemo) -> impl Future<Output = Result<Demo, DomainError>> + Send;

    /// 删除
    fn delete(&self, id: i64) -> impl Future<Output = Result<(), DomainError>> + Send;
}
