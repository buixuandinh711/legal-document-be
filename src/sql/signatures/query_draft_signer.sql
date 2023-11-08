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
        (
            SELECT "created_at",
                "assignee_address" as officer_address,
                "assignee_division_id" as division_id,
                "assignee_position_index" as position_index
            FROM "review_tasks"
            WHERE "draft_id" = $1
                AND (
                    "status" = 0
                    OR "status" = 1
                )
        )
        UNION
        (
            SELECT "created_at",
                "signer_address" as officer_address,
                "division_onchain_id" as division_id,
                "position_index"
            FROM "draft_signatures"
            WHERE "draft_id" = $1
        )
    ) pt ON pt."officer_address" = pd."officer_address"
    AND pt."division_id" = pd."division_onchain_id"
    AND pt."position_index" = pd."position_index"
    INNER JOIN "onchain_officers" of ON of."address" = pd."officer_address"
WHERE pt."created_at" IS NULL;