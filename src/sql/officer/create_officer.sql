INSERT INTO "officers" (
        "username",
        "password",
        "onchain_address",
        "private_key",
        "name",
        "date_of_birth",
        "sex",
        "status",
        "transaction_hash",
        "finalized"
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
        $9,
        $10
    );