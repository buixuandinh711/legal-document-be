CREATE TABLE "officials"(
	"id" BIGSERIAL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL UNIQUE,
	"address" VARCHAR(255) NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"date_of_birth" VARCHAR(255) NOT NULL,
	"sex" VARCHAR(255) NOT NULL,
	"status" SMALLINT NOT NULL,
	"password" VARCHAR(255) NOT NULL,
	"private_key" VARCHAR(255) NOT NULL
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
	"official_id" BIGINT NOT NULL,
	"onchain_id" BIGINT NOT NULL,
	"position_index" SMALLINT NOT NULL,
	"name" VARCHAR(255) NOT NULL,
	"role" SMALLINT NOT NULL,
	PRIMARY KEY("official_id", "onchain_id", "position_index"),
	CONSTRAINT "fk_position_official" FOREIGN KEY("official_id") REFERENCES "officials"("id"),
	CONSTRAINT "fk_position_division" FOREIGN KEY("onchain_id") REFERENCES "divisions"("id")
);