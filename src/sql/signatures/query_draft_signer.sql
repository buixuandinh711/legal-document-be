SELECT pd."officer_address",
    of."name" as officer_name,
    pd."position_index",
    pd."name" as position_name
FROM (
        SELECT *
        FROM "onchain_positions"
        WHERE "division_onchain_id" = (
                SELECT "division_onchain_id"
                FROM "drafts"
                WHERE "id" = $1
            )
    ) pd
    LEFT JOIN (
        SELECT "signer_address",
            "division_onchain_id",
            "position_index",
            "signature"
        FROM "draft_signatures"
        WHERE "draft_id" = $1
    ) ps ON ps."signer_address" = pd."officer_address"
    AND ps."division_onchain_id" = pd."division_onchain_id"
    AND ps."position_index" = pd."position_index"
    INNER JOIN "onchain_officers" of ON of."address" = pd."officer_address"
WHERE ps."signature" IS NULL;