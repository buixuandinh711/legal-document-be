SELECT of."name" AS "officer_name",
    of."status",
    op."position_index",
    op."name" as "position_name",
    op."role" as "posistion_role",
    op."division_onchain_id",
    od."name" AS "division_name"
FROM "onchain_officers" of
    INNER JOIN "onchain_positions" op ON of."address" = op."officer_address"
    INNER JOIN "onchain_divisions" od ON op."division_onchain_id" = od."onchain_id"
WHERE of."address" = $1
    AND of."status" = 1
    AND od."status" = 1
    AND op."role" != 0;