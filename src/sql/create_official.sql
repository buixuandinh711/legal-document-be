INSERT INTO "officials" (
        "username",
        "address",
        "name",
        "date_of_birth",
        "sex",
        "status",
        "password",
        "private_key"
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