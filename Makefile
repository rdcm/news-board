lint:
	cargo clippy

format:
	cargo fmt

build-dev:
	cargo build --all

docker-build:
	DOCKER_BUILDKIT=1 docker compose build --progress=plain --no-cache

schema:
	diesel print-schema --database-url postgres://postgres:postgres@localhost:5432/postgres > schema.rs
	mv schema.rs ./db-schema/src/schema.rs

docker-up:
	docker compose up -d

docker-down:
	docker compose down