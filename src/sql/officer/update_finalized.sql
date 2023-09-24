UPDATE "officers"
SET "finalized" = TRUE
WHERE "transaction_hash" = $1;