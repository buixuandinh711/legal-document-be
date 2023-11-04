DROP TRIGGER IF EXISTS "auto_update_drafts_updated_at" ON "drafts" CASCADE;
DROP TRIGGER IF EXISTS "auto_add_signatures_created_at" ON "draft_signatures" CASCADE;
DROP TABLE IF EXISTS "draft_signatures" CASCADE;
DROP TABLE IF EXISTS "drafts" CASCADE;
DROP TABLE IF EXISTS "officers" CASCADE;
DROP TABLE IF EXISTS "document_types" CASCADE;
DROP TABLE IF EXISTS "documents" CASCADE;
DROP TABLE IF EXISTS "review_tasks" CASCADE;
DROP FUNCTION IF EXISTS update_drafts_updated_at CASCADE;
DROP FUNCTION IF EXISTS add_signatures_created_at CASCADE;
--------------------------------------------------------------------------------------------
CREATE TABLE "officers"(
	"id" BIGSERIAL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL UNIQUE,
	"password" VARCHAR(255) NOT NULL,
	"onchain_address" VARCHAR(255) NOT NULL UNIQUE,
	"private_key" VARCHAR(255) NOT NULL
);
--------------------------------------------------------------------------------------------
CREATE TABLE "documents" (
	"id" BIGSERIAL PRIMARY KEY,
	"hash" VARCHAR(255) NOT NULL UNIQUE,
	"resource_uri" VARCHAR(255) NOT NULL
);
--------------------------------------------------------------------------------------------
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
--------------------------------------------------------------------------------------------
CREATE TABLE "drafts" (
	"id" BIGSERIAL PRIMARY KEY,
	"drafter_address" VARCHAR(42) NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"document_no" VARCHAR(255) NOT NULL,
	"document_name" VARCHAR(255) NOT NULL,
	"document_type" INT NOT NULL,
	"document_hash" VARCHAR(255) NOT NULL,
	"file_name" VARCHAR(255) NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	CONSTRAINT "fk_draft_officer" FOREIGN KEY("drafter_address") REFERENCES "officers"("onchain_address"),
	CONSTRAINT "fk_draft_doc_type" FOREIGN KEY("document_type") REFERENCES "document_types"("id")
);
------------------------------------------------
CREATE OR REPLACE FUNCTION update_drafts_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW."updated_at" = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
------------------------------------------------
CREATE TRIGGER "auto_update_drafts_updated_at" BEFORE
INSERT
	OR
UPDATE ON "drafts" FOR EACH ROW EXECUTE FUNCTION update_drafts_updated_at();
--------------------------------------------------------------------------------------------
CREATE TABLE "draft_signatures" (
	"draft_id" BIGINT NOT NULL,
	"signer_address" VARCHAR(255) NOT NULL,
	"division_onchain_id" VARCHAR(255) NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"signature" VARCHAR(255) NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	PRIMARY KEY(
		"draft_id",
		"signer_address",
		"division_onchain_id",
		"position_index"
	),
	CONSTRAINT "fk_sig_draft" FOREIGN KEY("draft_id") REFERENCES "drafts"("id"),
	CONSTRAINT "fk_sig_officer" FOREIGN KEY("signer_address") REFERENCES "officers"("onchain_address")
);
------------------------------------------------
CREATE OR REPLACE FUNCTION add_signatures_created_at() RETURNS TRIGGER AS $$ BEGIN NEW."created_at" = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
------------------------------------------------
CREATE TRIGGER "auto_add_signatures_created_at" BEFORE
INSERT ON "draft_signatures" FOR EACH ROW EXECUTE FUNCTION add_signatures_created_at();
--------------------------------------------------------------------------------------------
CREATE TABLE "review_tasks" (
	"id" BIGSERIAL PRIMARY KEY,
	"draft_id" BIGINT NOT NULL,
	"assigner_address" VARCHAR(255) NOT NULL,
	"assigner_division_id" VARCHAR(255) NOT NULL,
	"assinger_position_index" SMALLINT NOT NULL,
	"assignee_address" VARCHAR(255) NOT NULL,
	"assignee_division_id" VARCHAR(255) NOT NULL,
	"assingee_position_index" SMALLINT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"status" SMALLINT NOT NULL
);
------------------------------------------------
CREATE OR REPLACE FUNCTION add_review_task_created_at() RETURNS TRIGGER AS $$ BEGIN NEW."created_at" = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
------------------------------------------------
CREATE TRIGGER "auto_add_review_task_created_at" BEFORE
INSERT ON "review_tasks" FOR EACH ROW EXECUTE FUNCTION add_review_task_created_at();
--------------------------------------------------------------------------------------------
-- CREATE TABLE "onchain_officers"(
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"address" VARCHAR(255) NOT NULL UNIQUE,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"date_of_birth" VARCHAR(255) NOT NULL,
-- 	"sex" VARCHAR(255) NOT NULL,
-- 	"status" SMALLINT NOT NULL
-- );
-- --------------------------------------------------------------------------------------------
-- CREATE TABLE "onchain_divisions"(
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"onchain_id" VARCHAR(255) NOT NULL UNIQUE,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"supervisory_id" VARCHAR(255),
-- 	"status" SMALLINT NOT NULL
-- );
-- --------------------------------------------------------------------------------------------
-- CREATE TABLE "onchain_positions"(
-- 	"officer_address" VARCHAR(255) NOT NULL,
-- 	"division_onchain_id" VARCHAR(255) NOT NULL,
-- 	"position_index" SMALLINT NOT NULL,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"role" SMALLINT NOT NULL,
-- 	PRIMARY KEY(
-- 		"officer_address",
-- 		"division_onchain_id",
-- 		"position_index"
-- 	)
-- );
-- --------------------------------------------------------------------------------------------
-- CREATE TABLE IF NOT EXISTS "onchain_documents" (
-- 	"id" BIGSERIAL PRIMARY KEY,
-- 	"document_content_hash" VARCHAR(255) NOT NULL UNIQUE,
-- 	"number" VARCHAR(255) NOT NULL,
-- 	"name" VARCHAR(255) NOT NULL,
-- 	"doc_type" VARCHAR(255) NOT NULL,
-- 	"division_id" VARCHAR(255) NOT NULL,
-- 	"published_timestamp" INT NOT NULL,
-- 	"publisher_address" VARCHAR(255) NOT NULL,
-- 	"publisher_division_id" VARCHAR(255) NOT NULL,
-- 	"publisher_position_index" SMALLINT NOT NULL
-- );
-- --------------------------------------------------------------------------------------------
-- CREATE TABLE IF NOT EXISTS "onchain_document_signatures" (
-- 	"document_content_hash" VARCHAR(255) NOT NULL,
-- 	"signers_address" VARCHAR(255) NOT NULL,
-- 	"division_onchain_id" VARCHAR(255) NOT NULL,
-- 	"position_index" SMALLINT NOT NULL,
-- 	PRIMARY KEY(
-- 		"document_content_hash",
-- 		"signers_address",
-- 		"division_onchain_id",
-- 		"position_index"
-- 	)
-- );