
all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@cargo build --release

.PHONY: test 
test:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in debug mode..."
	@cargo build 

.PHONY: test_install
test_install:
	@echo ":: Installing binaries..."
	@install -Dm 755 target/release/penrose-wm /usr/bin/penrose-wm
	@echo ":: Done"

.PHONY: install
install:
	@echo ":: Installing binaries..."
	@install -Dm 755 target/release/penrose-wm /usr/bin/penrose-wm
	@install -Dm 755 target/release/redblocks /usr/bin/redblocks
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@rm -f /usr/bin/penrose-wm
	@rm -f /usr/bin/redblocks
	@echo ":: Done"
