table "atlas_schema_revisions" {
  schema = schema.atlas_schema_revisions
  column "version" {
    null = false
    type = character_varying
  }
  column "description" {
    null = false
    type = character_varying
  }
  column "type" {
    null    = false
    type    = bigint
    default = 2
  }
  column "applied" {
    null    = false
    type    = bigint
    default = 0
  }
  column "total" {
    null    = false
    type    = bigint
    default = 0
  }
  column "executed_at" {
    null = false
    type = timestamptz
  }
  column "execution_time" {
    null = false
    type = bigint
  }
  column "error" {
    null = true
    type = text
  }
  column "error_stmt" {
    null = true
    type = text
  }
  column "hash" {
    null = false
    type = character_varying
  }
  column "partial_hashes" {
    null = true
    type = jsonb
  }
  column "operator_version" {
    null = false
    type = character_varying
  }
  primary_key {
    columns = [column.version]
  }
}
table "users" {
  schema = schema.public
  column "id" {
    null = false
    type = serial
  }
  column "email" {
    null = false
    type = character_varying
  }
  column "first_name" {
    null = false
    type = character_varying
  }
  column "last_name" {
    null = false
    type = character_varying
  }
  column "role" {
    null = false
    type = character_varying
  }
  column "created_at" {
    null    = false
    type    = timestamp
    default = sql("now()")
  }
  column "updated_at" {
    null = true
    type = timestamp
  }
  primary_key {
    columns = [column.id]
  }
}
schema "atlas_schema_revisions" {
}
schema "public" {
}
