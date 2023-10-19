CREATE TABLE "officers"(
	"id" BIGSERIAL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL UNIQUE,
	"password" VARCHAR(255) NOT NULL,
	"onchain_address" VARCHAR(255) NOT NULL UNIQUE,
	"private_key" VARCHAR(255) NOT NULL
);
CREATE TABLE "documents" (
	"id" BIGSERIAL PRIMARY KEY,
	"hash" VARCHAR(255) NOT NULL UNIQUE,
	"resource_uri" VARCHAR(255) NOT NULL
);
----------------------------------------------------------------
CREATE TABLE "document_types" ("id" SERIAL PRIMARY KEY, "name" VARCHAR(255));
INSERT INTO "document_types" (name)
VALUES ('Hiến pháp'),
	('Bộ luật'),
	('Luật'),
	('Nghị quyết'),
	('Pháp lệnh'),
	('Quyết định'),
	('Nghị định'),
	('Thông tư'),
	('Nghị quyết liên tịch');
CREATE TABLE "drafts" (
	"id" BIGSERIAL PRIMARY KEY,
	"drafter" BIGINT NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"document_no" VARCHAR(255) NOT NULL,
	"document_name" VARCHAR(255) NOT NULL,
	"document_type" INT NOT NULL,
	"document_hash" VARCHAR(255) NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	CONSTRAINT "fk_draft_officer" FOREIGN KEY("drafter") REFERENCES "officers"("id"),
	CONSTRAINT "fk_draft_doc_type" FOREIGN KEY("document_type") REFERENCES "document_types"("id")
);
-- CREATE OR REPLACE FUNCTION update_drafts_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW."updated_at" = NOW();
-- RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;
CREATE TRIGGER "auto_update_drafts_updated_at" BEFORE
INSERT
	OR
UPDATE ON "drafts" FOR EACH ROW EXECUTE FUNCTION update_drafts_updated_at();
----------------------------------------------------------------
CREATE TABLE "onchain_officers"(
	"id" BIGSERIAL PRIMARY KEY,
	"address" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"date_of_birth" VARCHAR(255) NOT NULL,
	"sex" VARCHAR(255) NOT NULL,
	"status" SMALLINT NOT NULL
);
CREATE TABLE "onchain_divisions"(
	"id" BIGSERIAL PRIMARY KEY,
	"onchain_id" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"supervisory_id" VARCHAR(255),
	"status" SMALLINT NOT NULL
);
CREATE TABLE "onchain_positions"(
	"officer_address" VARCHAR(255) NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"role" SMALLINT NOT NULL,
	PRIMARY KEY(
		"officer_address",
		"division_onchain_id",
		"position_index"
	)
);
CREATE TABLE "onchain_documents" (
	"id" BIGSERIAL PRIMARY KEY,
	"hash" VARCHAR(255) NOT NULL UNIQUE,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"submitter_address" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"signers_address" VARCHAR(255) [] NOT NULL
);