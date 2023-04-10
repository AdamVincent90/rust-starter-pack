# Binary commands contain the cargo functionality to run your binaries using the --bin target followed
# by your binary name (these are defined within your cargo.toml file)

# ==============================================================================
# Binary Commands
rust-web-api:
	cargo run --bin rust-web-api

rsa-keypair:
	cargo run --bin ssl

# ==============================================================================
# Docker Compose

## Run a bundles rust app service - targetting your main binary.
run-dev:
	docker compose  \
		-p ultimate-rust-service \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		up \
		--build

## Stop the docker container from running.
stop-dev:
	docker compose  \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		down \
		--rmi local

# ==============================================================================
# Lumber command
.PHONY: lumber
lumber:
	cargo run --bin lumber $(filter-out $@,$(MAKECMDGOALS))

# ==============================================================================
# DB Migrations

# Creates a new manual migration .sql file in the migrations folder for you to manually add SQL to
# This should be used for data migrations and stored functions/procedures, not schema migrations
db-migrate-new:
	atlas migrate new $(name) --dir "file://./scaffold/migrations"

# Recalculate the hash of the migration folder 
# see here: https://atlasgo.io/concepts/migration-directory-integrity
db-migrate-hash:
	atlas migrate hash --dir "file://./scaffold/migrations"

# Takes any schema changes you've made to your local database, updates the schema.hcl file and then adds the migration to the migrations folder
# This will automatically update the migration hash (atlas.sum)
db-schema-update:
	atlas schema inspect -u "postgres://postgres:example@127.0.0.1:5439/postgres?sslmode=disable" > ./scaffold/migrations/schema.hcl
	atlas migrate diff $(name) --dir "file://scaffold/migrations" --to "file://scaffold/migrations/schema.hcl" --dev-url "docker://postgres/14"