extern crate autocfg;

fn main() {
    let cfg = autocfg::new();
    cfg.emit_path_cfg("core::task::Poll", "has_task_poll");
    cfg.emit_path_cfg("std::task::Poll", "has_task_poll");

    // Needed for `assert_matches!` and `?` macro repetition
    // See https://doc.rust-lang.org/edition-guide/rust-2018/macros/at-most-once.html
    cfg.emit_rustc_version(1, 32);
}
