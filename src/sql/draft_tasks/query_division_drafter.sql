SELECT op."officer_address",
    of."name" as officer_name,
    op."position_index",
    op."name" as position_name
FROM "onchain_positions" op
    INNER JOIN "onchain_officers" of ON of."address" = op."officer_address"
WHERE op."division_onchain_id" = $1
    AND NOT (
        op."division_onchain_id" = $1
        AND op."officer_address" = $2
        AND op."position_index" = $3
    );