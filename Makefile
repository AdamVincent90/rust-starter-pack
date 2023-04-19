# ==============================================================================
# Install dependencies
init:
	brew install protobuf
	brew install ariga/tap/atlas
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	cargo check

# ==============================================================================
# Binary Commands

# Binary commands contain the cargo functionality to run your binaries using the --bin target followed
# by your binary name (these are defined within your cargo.toml file)

# Run web api locally (Check envs)
rust-web-api:
	RUST_LOG=debug cargo run --bin rust-web-api

# Create a new RSA256 keypair
.PHONY: rsa-keypair
rsa-keypair:
	cargo run --bin ssl keygen $(filter-out $@,$(MAKECMDGOALS))

# Create a new access token using a local rsa keypair
.PHONY: token
token:
	cargo run --bin ssl token $(filter-out $@,$(MAKECMDGOALS))

# Generate core custom functionality.
.PHONY: lumber
lumber:
	cargo run --bin lumber $(filter-out $@,$(MAKECMDGOALS))

# ==============================================================================
# Docker Compose

## Run a bundled rust app service - targetting your main binary.
docker-up:
	docker compose  \
		-p ultimate-rust-service \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		up \
		--build

## Stop the docker container from running.
docker-down:
	docker compose  \
		-f scaffold/docker-compose/docker-compose.yaml \
		--env-file .env \
		down \
		--rmi local


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