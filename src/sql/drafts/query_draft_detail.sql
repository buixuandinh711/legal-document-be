SELECT d."id",
    o."username" as drafter_username,
    oo."name" AS drafter_name,
    d."name",
    d."document_name",
    d."updated_at"
FROM "drafts" d
    INNER JOIN "officers" o ON d."drafter" = o."id"
    INNER JOIN "onchain_officers" oo ON o."onchain_address" = oo."address"
    INNER JOIN "onchain_positions" op ON o."onchain_address" = op."officer_address"
WHERE o."id" = $1
    AND op."division_onchain_id" = $2
    AND op."position_index" = $3;