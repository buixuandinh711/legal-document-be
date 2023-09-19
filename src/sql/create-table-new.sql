CREATE TABLE "positions"(
    "official_id" BIGINT NOT NULL,
    "division_id" VARCHAR(255) NOT NULL,
    "position_index" SMALLINT NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "role" SMALLINT NOT NULL
);
ALTER TABLE "positions"
ADD PRIMARY KEY("official_id");

ALTER TABLE "positions"
ADD PRIMARY KEY("division_id");

ALTER TABLE "positions"
ADD PRIMARY KEY("position_index");

CREATE TABLE "officials"(
    "id" BIGINT NOT NULL,
    "address" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "date_of_birth" VARCHAR(255) NOT NULL,
    "sex" VARCHAR(255) NOT NULL,
    "status" SMALLINT NOT NULL
);

ALTER TABLE "officials"
ADD PRIMARY KEY("id");

ALTER TABLE "officials"
ADD CONSTRAINT "officials_address_unique" UNIQUE("address");

CREATE TABLE "divisions"(
    "id" BIGINT NOT NULL,
    "division_id" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "supervisory_id" BIGINT NOT NULL,
    "status" SMALLINT NOT NULL
);

ALTER TABLE "divisions"
ADD PRIMARY KEY("id");

ALTER TABLE "divisions"
ADD CONSTRAINT "divisions_division_id_unique" UNIQUE("division_id");

ALTER TABLE "positions"
ADD CONSTRAINT "positions_official_id_foreign" FOREIGN KEY("official_id") REFERENCES "officials"("id");

ALTER TABLE "positions"
ADD CONSTRAINT "positions_division_id_foreign" FOREIGN KEY("division_id") REFERENCES "divisions"("id");

ALTER TABLE "divisions"
ADD CONSTRAINT "divisions_supervisory_id_foreign" FOREIGN KEY("supervisory_id") REFERENCES "divisions"("id");