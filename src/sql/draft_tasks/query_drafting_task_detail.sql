SELECT "name",
    "assigner_address",
    "assigner_division_id",
    "assigner_position_index",
    "assignee_address",
    "assignee_division_id",
    "assignee_position_index",
    "created_at",
    "draft_id"
FROM "draft_tasks"
WHERE "id" = $1;