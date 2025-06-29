.PHONY: build install remove

SERVICE_NAME=discord-package-watcher.service
SYSTEMD_USER_DIR=$(HOME)/.config/systemd/user
DOWNLOAD_DIR=$(HOME)/Downloads/discord

install: build
	cargo install --path .
	mkdir -p $(SYSTEMD_USER_DIR)
	mkdir -p $(DOWNLOAD_DIR)
	cp systemd/$(SERVICE_NAME) $(SYSTEMD_USER_DIR)/
	systemctl --user daemon-reload
	systemctl --user enable $(SERVICE_NAME) || true
	systemctl --user is-active --quiet $(SERVICE_NAME) && \
		systemctl --user restart $(SERVICE_NAME) || \
		systemctl --user start $(SERVICE_NAME)

remove:
	systemctl --user disable --now $(SERVICE_NAME) || true
	rm -f $(SYSTEMD_USER_DIR)/$(SERVICE_NAME)
	systemctl --user daemon-reload
