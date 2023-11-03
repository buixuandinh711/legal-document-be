SELECT f."onchain_address",
    f."password"
FROM "officers" f
    INNER JOIN "onchain_officers" of ON of."address" = f."onchain_address"
WHERE of."status" = 1
    AND f."username" = $1;