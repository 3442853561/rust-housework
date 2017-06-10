REM This batch will help you update Rust when you network is not "VERY WELL"
SET RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
SET RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
TITLE Rust Update
CLS
rustup update
PAUSE
