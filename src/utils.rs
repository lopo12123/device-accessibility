/// 辅助键 (ctrl / shift / alt 中的 0/1/2/3 个)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ExtraKey {
    pub ctrl: Option<bool>,
    pub alt: Option<bool>,
    pub shift: Option<bool>,
}

/// 组合键情况 (目标键 + 辅助键)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyCombination {
    /// 目标键 (可用值见 mapper 文件)
    pub key: String,
    /// 辅助键 见[ExtraKey]
    pub extra: Option<ExtraKey>,
}

/// 坐标
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct MouseLocation {
    /// x 方向 (`i32`)
    pub x: i32,
    /// y 方向 (`i32`)
    pub y: i32,
}

