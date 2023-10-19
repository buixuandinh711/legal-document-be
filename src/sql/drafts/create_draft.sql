INSERT INTO "drafts" (
        "drafter",
        "division_id",
        "position_index",
        "name",
        "document_no",
        "document_name",
        "document_type",
        "document_hash"
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8
    );