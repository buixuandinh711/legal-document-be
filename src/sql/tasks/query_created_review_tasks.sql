SELECT rt."id",
    d."name" as draft_name,
    of."name" as officer_name,
    op."name" as position_name,
    rt."created_at",
    rt."status"
FROM "review_tasks" rt
    INNER JOIN "drafts" d ON d."id" = rt."draft_id"
    INNER JOIN "onchain_officers" of ON of."address" = rt."assignee_address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = rt."assignee_address"
    AND op."division_onchain_id" = rt."assignee_division_id"
    AND op."position_index" = rt."assingee_position_index"
WHERE rt."assigner_address" = $1
    AND rt."assigner_division_id" = $2
    AND rt."assinger_position_index" = $3