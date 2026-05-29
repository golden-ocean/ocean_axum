/// 统一的分页请求参数
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl Pagination {
    pub fn limit(&self) -> i64 {
        // 1. 先用 max(1) 斩断前端传 0 的恶作剧：如果传 0，强制变成 1
        // 2. 再用 min(100) 封死黑客大宗捞数的胃口：如果传 99999，强制压回 100
        self.page_size.max(1).min(100) as i64
    }

    pub fn offset(&self) -> i64 {
        // 页码防自残：哪怕传 page = 0，也强制修正为 1
        let p = self.page.max(1);
        let size = self.page_size.max(1);

        ((p - 1) * size) as i64
    }
}

/// 统一的分页响应结果封装
#[derive(Debug, Clone)]
pub struct PageRes<T> {
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

impl<T> PageRes<T> {
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
    pub fn map<F, U>(self, f: F) -> PageRes<U>
    where
        F: FnMut(T) -> U,
    {
        PageRes {
            list: self.list.into_iter().map(f).collect(),
            total: self.total,
            page: self.page,
            page_size: self.page_size,
            total_pages: self.total_pages,
        }
    }
}
