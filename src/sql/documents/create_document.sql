INSERT INTO "documents"("hash", "resource_uri")
VALUES ($1, $2) ON CONFLICT ("hash") DO NOTHING;