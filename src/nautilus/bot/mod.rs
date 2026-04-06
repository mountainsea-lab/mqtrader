use anyhow::Context;
use std::env;
use std::path::Path;
pub mod mqtrader;

/// 获取策略动态库路径（自动适配平台扩展名）
fn get_strategy_lib_path() -> anyhow::Result<String> {
    let base_path =
        env::var("STRATEGY_LIB").context("STRATEGY_LIB environment variable not set")?;

    let path = Path::new(&base_path);

    // 如果已有扩展名，直接返回
    if path.extension().is_some() {
        return Ok(base_path);
    }

    // 否则添加平台特定扩展名
    let mut path_buf = path.to_path_buf();
    #[cfg(target_os = "linux")]
    path_buf.set_extension("so");
    #[cfg(target_os = "macos")]
    path_buf.set_extension("dylib");
    #[cfg(target_os = "windows")]
    path_buf.set_extension("dll");

    path_buf
        .to_str()
        .map(|s| s.to_string())
        .context("Failed to convert path to string")
}
