/// 统一的分页请求参数
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

impl Pagination {
    pub fn new(page: Option<u64>, page_size: Option<u64>) -> Self {
        Self {
            page: page.unwrap_or(1).max(1),
            page_size: page_size.unwrap_or(20).clamp(1, 100),
        }
    }

    pub fn limit(self) -> i64 {
        self.page_size as i64
    }

    pub fn offset(self) -> i64 {
        ((self.page - 1) * self.page_size) as i64
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new(None, None)
    }
}

/// 统一的分页响应结果封装
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct Page<T> {
    /// 列表数据
    pub list: Vec<T>,
    /// 总记录数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub page_size: u64,
    /// 总页数 (根据 total 和 page_size 自动计算)
    pub total_pages: u64,
}

impl<T> Page<T> {
    pub fn new(total: u64, page: u64, page_size: u64, list: Vec<T>) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            (total + page_size - 1) / page_size
        };
        Self {
            list,
            total,
            page,
            page_size,
            total_pages,
        }
    }

    /// 利用泛型映射，在表现层实现流水线式的 Dto 转换
    pub fn map<F, U>(self, f: F) -> Page<U>
    where
        F: FnMut(T) -> U,
    {
        Page {
            list: self.list.into_iter().map(f).collect(),
            total: self.total,
            page: self.page,
            page_size: self.page_size,
            total_pages: self.total_pages,
        }
    }
}
