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
        SELECT "id",
            "assignee_address",
            "assignee_division_id",
            "assingee_position_index"
        FROM "review_tasks"
        WHERE "draft_id" = $1
    ) pt ON pt."assignee_address" = pd."officer_address"
    AND pt."assignee_division_id" = pd."division_onchain_id"
    AND pt."assingee_position_index" = pd."position_index"
    INNER JOIN "onchain_officers" of ON of."address" = pd."officer_address"
WHERE pt."id" IS NULL;