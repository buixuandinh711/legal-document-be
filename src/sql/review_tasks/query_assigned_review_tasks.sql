SELECT rt."id",
    d."name" as draft_name,
    of."name" as officer_name,
    op."name" as position_name,
    rt."created_at",
    rt."status"
FROM "review_tasks" rt
    INNER JOIN "drafts" d ON d."id" = rt."draft_id"
    INNER JOIN "onchain_officers" of ON of."address" = rt."assigner_address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = rt."assigner_address"
    AND op."division_onchain_id" = rt."assigner_division_id"
    AND op."position_index" = rt."assigner_position_index"
WHERE rt."assignee_address" = $1
    AND rt."assignee_division_id" = $2
    AND rt."assignee_position_index" = $3