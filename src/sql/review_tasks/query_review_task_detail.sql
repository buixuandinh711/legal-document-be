SELECT "draft_id",
    "assigner_address",
    "assigner_division_id",
    "assigner_position_index",
    "assignee_address",
    "assignee_division_id",
    "assignee_position_index",
    "created_at",
    "status"
FROM "review_tasks"
WHERE "id" = $1;