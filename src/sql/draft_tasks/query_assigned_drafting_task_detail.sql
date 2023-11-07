SELECT dt."name" as task_name,
    of."name" as assigner_name,
    op."name" as assigner_position_name,
    dt."created_at",
    dt."draft_id"
FROM "draft_tasks" dt
    INNER JOIN "onchain_officers" of ON of."address" = dt."assigner_address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = dt."assigner_address"
    AND op."division_onchain_id" = dt."assigner_division_id"
    AND op."position_index" = dt."assigner_position_index"
WHERE dt."id" = $1