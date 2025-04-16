dev:
	POSTGRES_TUI_LOG=debug cargo run --bin postgres_tui

log:
	tail -f postgres_tui.log
