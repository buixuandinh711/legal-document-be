SELECT dt."id",
    dt."name" as task_name,
    of."name" as assignee_name,
    op."name" as assignee_position_name,
    dt."created_at",
    dt."draft_id"
FROM "draft_tasks" dt
    INNER JOIN "onchain_officers" of ON of."address" = dt."assignee_address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = dt."assignee_address"
    AND op."division_onchain_id" = dt."assignee_division_id"
    AND op."position_index" = dt."assingee_position_index"
WHERE dt."assigner_address" = $1
    AND dt."assigner_division_id" = $2
    AND dt."assinger_position_index" = $3;