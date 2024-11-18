lint:
	cargo clippy

format:
	cargo fmt

build-dev:
	cargo build --all

docker-build:
	DOCKER_BUILDKIT=1 docker compose build --progress=plain --no-cache

docker-up:
	docker compose up -d

docker-down:
	docker compose down

deploy-k8s-dev:
	helm upgrade --install --atomic --timeout 300s --wait news-board helm/news-board -f ./helm/news-board/values/dev.yaml --create-namespace --namespace news-board

delete-k8s:
	helm delete news-board --namespace news-board

render-k8s-dev:
	helm template -f ./helm/news-board/values/dev.yaml helm/news-board > template-render-dev.yaml --debug