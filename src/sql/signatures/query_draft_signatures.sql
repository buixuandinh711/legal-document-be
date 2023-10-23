SELECT s."id",
    of."name" as signer_name
FROM "signatures" s
    INNER JOIN "officers" o ON s."signer_id" = o."id"
    INNER JOIN "onchain_officers" of ON o."onchain_address" = of."address"
WHERE s."draft_id" = $1;