# For quick iterational development
1) cargo check - checks the code without producing final executable
2) cargo build - compile
3) .\target\debug\game.exe - run

# For sync after Cargo.toml update
1) cargo update - dependency resolution and lockfile syncing
2) cargo check
3) cargo generate-lockfile - optional, if cargo says the lockfile is out of date

# For checking warnings and formatting
1) cargo clippy -- -D warnings
2) cargo fmt --check



