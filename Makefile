# ==============================================================================
# Docker Compose

run-dev:
	docker compose  \
		-p ultimate-rust-service \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		up \
		--build

stop-dev:
	docker compose  \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		down \
		--rmi local