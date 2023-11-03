INSERT INTO "drafts" (
        "drafter_address",
        "division_onchain_id",
        "position_index",
        "name",
        "document_no",
        "document_name",
        "document_type",
        "document_hash",
        "file_name"
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9
    )
RETURNING id;