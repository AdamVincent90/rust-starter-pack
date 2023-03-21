-- create "users" table
CREATE TABLE "public"."users" ("id" serial NOT NULL, "email" character varying NOT NULL, "first_name" character varying NOT NULL, "last_name" character varying NOT NULL, "role" character varying NOT NULL, "created_at" timestamp NOT NULL DEFAULT now(), "updated_at" timestamp NULL, PRIMARY KEY ("id"));

INSERT INTO "public"."users" ("id", "email", "first_name", "last_name", "role", "created_at", "updated_at") VALUES
(1, 'admin@example.com', 'admin', 'user', 'admin', '2023-03-21 11:43:31.675651', NULL);