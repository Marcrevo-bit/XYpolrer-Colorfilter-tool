// Tauri 编译期构建钩子（由 build-dependencies 里的 tauri-build 调用）。
// 它读取 tauri.conf.json 与 capabilities 配置，生成运行期所需的代码/权限清单，
// 并在每次 cargo build 时根据配置变化重新生成（类似 autotools 的 configure 阶段）。
// 本身无业务逻辑，保持极简即可。
fn main() {
    tauri_build::build()
}
