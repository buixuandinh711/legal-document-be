SELECT (
        SELECT of."status"
        FROM "onchain_officers" of
        WHERE of."address" = $1
    ) AS "officer_status",
    (
        SELECT od."status"
        FROM "onchain_divisions" od
        WHERE od."onchain_id" = $2
    ) AS div_status,
    (
        SELECT op."role"
        FROM "onchain_positions" op
        WHERE op."officer_address" = $1
            AND op."division_onchain_id" = $2
            AND op."position_index" = $3
    ) AS pos_role;