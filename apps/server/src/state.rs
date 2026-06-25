use iam::presentation::web::IamSliceState;

/// 全局唯一状态
#[derive(Clone)]
pub struct AppState {
    pub iam_slice_state: IamSliceState,
    // pub sys_slice: Arc<SysAppState>,
    // pub order_slice: Arc<OrderAppState>,
}

impl AppState {
    /// 组装厂专属构造函数：负责执行各个模块原材料的统一归仓打包
    pub fn new(iam_slice_state: IamSliceState) -> Self {
        Self { iam_slice_state }
    }
}
