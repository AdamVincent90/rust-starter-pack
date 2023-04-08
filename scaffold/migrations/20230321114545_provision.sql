-- create "users" table
CREATE TABLE "public"."users" ("id" serial NOT NULL, "email" character varying NOT NULL, "first_name" character varying NOT NULL, "last_name" character varying NOT NULL, "role" character varying NOT NULL, "created_at" timestamp NOT NULL DEFAULT now(), "updated_at" timestamp NULL, PRIMARY KEY ("id"));

-- create "audit_logs" table
CREATE TABLE "public"."audit_logs" ("id" serial NOT NULL, "user_agent" character varying(255) NULL, "web_path" character varying(50) NULL, "host_address" character varying(30) NULL, "origin_ip_address" character varying(30) NULL, "request_uuid" character varying(36) NULL, "status_code" character varying(3) NULL, "created_at" timestamp NOT NULL DEFAULT now(), PRIMARY KEY ("id"));

INSERT INTO "public"."users" ("id", "email", "first_name", "last_name", "role", "created_at", "updated_at") VALUES
(1, 'admin@example.com', 'admin', 'user', 'admin', '2023-03-21 11:43:31.675651', NULL);