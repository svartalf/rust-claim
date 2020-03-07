fn main() {
    let cfg = autocfg::new();
    cfg.emit_path_cfg("core::task::Poll", "has_task_poll");
    cfg.emit_path_cfg("std::task::Poll", "has_task_poll");
}
