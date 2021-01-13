build:
	cargo build --release --target=armv7-unknown-linux-gnueabihf

dev:
	cargo build


DEVICE_IP ?= www.theoxao.com
DEVICE_HOST ?= root@$(DEVICE_IP)
deploy:
	ssh $(DEVICE_HOST) 'killall -q -9 mast || true'
	scp ./target/release/mast $(DEVICE_HOST):/data/service
	ssh $(DEVICE_HOST) '/data/service/start.sh'


run: dev deploy
