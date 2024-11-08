format:
	cargo fmt

build-dev:
	cargo build --all

schema:
	diesel print-schema --database-url postgres://postgres:postgres@localhost:5432/postgres > schema.rs
	mv schema.rs ./db-schema/src/schema.rs

docker-up:
	docker compose up -d

docker-down:
	docker compose down