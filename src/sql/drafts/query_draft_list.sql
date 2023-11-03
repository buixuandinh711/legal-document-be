SELECT d."id",
    o."username" as drafter_username,
    oo."name" AS drafter_name,
    d."name",
    d."document_name",
    d."updated_at"
FROM "drafts" d
    INNER JOIN "onchain_officers" oo ON oo."address" = d."drafter_address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = d."drafter_address"
    INNER JOIN "officers" o ON o."onchain_address" = d."drafter_address"
WHERE d."drafter_address" = $1
    AND op."division_onchain_id" = $2
    AND op."position_index" = $3;