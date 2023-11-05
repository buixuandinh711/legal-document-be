INSERT INTO "draft_signatures"(
        "draft_id",
        "signer_address",
        "division_onchain_id",
        "position_index",
        "signature"
    )
VALUES ($1, $2, $3, $4, $5);