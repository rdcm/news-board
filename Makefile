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

deploy-news-board-dev:
	helm upgrade --install --atomic --timeout 300s --wait news-board helm/news-board -f ./helm/news-board/values/dev.yaml --create-namespace --namespace news-board

delete-news-board:
	helm delete news-board --namespace news-board

render-news-board-dev:
	helm template -f ./helm/news-board/values/dev.yaml helm/news-board > template-render-dev.yaml --debug