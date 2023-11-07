INSERT INTO "draft_tasks" (
        "name",
        "assigner_address",
        "assigner_division_id",
        "assinger_position_index",
        "assignee_address",
        "assignee_division_id",
        "assingee_position_index"
    )
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id;