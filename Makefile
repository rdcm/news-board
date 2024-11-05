build-dev:
	cargo build --all

docker-up:
	docker compose up -d

docker-down:
	docker compose down