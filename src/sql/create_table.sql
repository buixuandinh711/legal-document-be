CREATE TABLE "officers"(
	"id" BIGSERIAL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL UNIQUE,
	"password" VARCHAR(255) NOT NULL,
	"onchain_address" VARCHAR(255) NOT NULL UNIQUE,
	"private_key" VARCHAR(255) NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"date_of_birth" VARCHAR(255) NOT NULL,
	"sex" VARCHAR(255) NOT NULL,
	"status" SMALLINT NOT NULL,
	"transaction_hash" VARCHAR(255) NOT NULL,
	"finalized" BOOLEAN NOT NULL
);
CREATE TABLE "divisions"(
	"id" BIGSERIAL PRIMARY KEY,
	"onchain_id" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"supervisory_id" BIGINT,
	"status" SMALLINT NOT NULL,
	CONSTRAINT "fk_supervisory" FOREIGN KEY("supervisory_id") REFERENCES "divisions"("id")
);
CREATE TABLE "positions"(
	"officer_id" BIGINT NOT NULL,
	"division_id" BIGINT NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"role" SMALLINT NOT NULL,
	PRIMARY KEY("officer_id", "division_id", "position_index"),
	CONSTRAINT "fk_position_officer" FOREIGN KEY("officer_id") REFERENCES "officers"("id"),
	CONSTRAINT "fk_position_division" FOREIGN KEY("division_id") REFERENCES "divisions"("id")
);