-- CREATE TABLE "officers"(
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"username" VARCHAR(255) NOT NULL UNIQUE,
-- 	"password" VARCHAR(255) NOT NULL,
-- 	"onchain_address" VARCHAR(255) NOT NULL UNIQUE,
-- 	"private_key" VARCHAR(255) NOT NULL
-- );
-- CREATE TABLE "onchain_officers"(
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"onchain_address" VARCHAR(255) NOT NULL UNIQUE,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"date_of_birth" VARCHAR(255) NOT NULL,
-- 	"sex" VARCHAR(255) NOT NULL,
-- 	"status" SMALLINT NOT NULL
-- );
-- CREATE TABLE "divisions"(
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"onchain_id" VARCHAR(255) NOT NULL UNIQUE,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"supervisory_id" BIGINT,
-- 	"status" SMALLINT NOT NULL,
-- 	CONSTRAINT "fk_supervisory" FOREIGN KEY("supervisory_id") REFERENCES "divisions"("id")
-- );
-- CREATE TABLE "positions"(
-- 	"onchain_officer_id" BIGINT NOT NULL,
-- 	"division_id" BIGINT NOT NULL,
-- 	"position_index" SMALLINT NOT NULL,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"role" SMALLINT NOT NULL,
-- 	PRIMARY KEY(
-- 		"onchain_officer_id",
-- 		"division_id",
-- 		"position_index"
-- 	),
-- 	CONSTRAINT "fk_position_onchain_officer" FOREIGN KEY("onchain_officer_id") REFERENCES "onchain_officers"("id"),
-- 	CONSTRAINT "fk_position_division" FOREIGN KEY("division_id") REFERENCES "divisions"("id")
-- );
CREATE TABLE "submitted_docs" (
	"id" BIGSERIAL PRIMARY KEY,
	"onchain_hash" VARCHAR(255) NOT NULL UNIQUE,
	"resource_uri" VARCHAR(255) NOT NULL
);
CREATE TABLE "onchain_submitted_docs" (
	"id" BIGSERIAL PRIMARY KEY,
	"onchain_hash" VARCHAR(255) NOT NULL UNIQUE,
	"division_id" BIGINT NOT NULL,
	"onchain_officer_id" BIGINT NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"signer_onchain_id" BIGINT [] NOT NULL,
	CONSTRAINT "fk_ocsd_sd" FOREIGN KEY("onchain_hash") REFERENCES "submitted_docs"("onchain_hash"),
	CONSTRAINT "fk_ocsd_division" FOREIGN KEY("division_id") REFERENCES "divisions"("id"),
	CONSTRAINT "fk_ocsd_of" FOREIGN KEY("onchain_officer_id") REFERENCES "onchain_officers"("id")
);