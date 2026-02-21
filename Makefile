.PHONY: build install clean gui gui-build

build:
	@echo "🔨 Building pure Rust GUI..."
	@cd gui-egui && cargo build --release
	@cp gui-egui/target/release/ansible-provisioning-gui ./ansible-gui
	@chmod +x ./ansible-gui
	@echo "✅ Build complete! Binary: ./ansible-gui"

install: build
	@echo "✅ Binary ready at ./ansible-gui"
	@echo ""
	@echo "Usage:"
	@echo "  ./ansible-gui     - Run GUI"
	@echo "  make gui          - Run GUI via make"

gui:
	@echo "🔄 Killing any existing instances..."
	@pkill -9 ansible-provisioning-gui 2>/dev/null || true
	@pkill -9 ansible-gui 2>/dev/null || true
	@sleep 0.5
	@echo "🚀 Launching GUI..."
	@cd gui-egui && cargo run --release

gui-build: build

clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd gui-egui && cargo clean
	@rm -f ansible-gui bin/ansible-provisioning-gui
	@echo "✅ Clean complete!"
