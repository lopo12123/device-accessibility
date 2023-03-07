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

/// 按键事件 (目标键 + 辅助键 + 按键状态)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyEv {
    /// 目标键 (可用值见 mapper 文件)
    pub key: String,
    /// 辅助键 见[ExtraKey]
    pub extra: Option<ExtraKey>,
    /// 是否是按下状态 (默认为 `false`)
    pub down: Option<bool>,
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

