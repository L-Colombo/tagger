VERSION=v0.1.0
NAME=tagger
EXEC=tgr
INSTALL_DIR=$(HOME)/.local/bin

default: build_release

clean:
	@echo "Removing build artifacts..."
	@cargo clean --quiet
	@echo "Done!"

build_release:
	@echo "Building $(NAME) version $(VERSION)..."
	@cargo build --release --quiet
	@echo "Finished building $(NAME) version $(VERSION)"

install: build_release
	@echo "Installing $(NAME) version $(VERSION)..."
	@cp target/release/$(EXEC) $(INSTALL_DIR)
	@echo "$(NAME) version $(VERSION) successfully installed"

uninstall:
	@echo "Uninstalling $(NAME) version $(VERSION)..."
	@rm $(INSTALL_DIR)/$(EXEC)
	@echo "$(NAME) version $(VERSION) uninstalled"
