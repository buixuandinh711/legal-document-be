SELECT of."name" AS "officer_name",
    of."status",
    op."position_index",
    op."name" as "position_name",
    op."role" as "posistion_role",
    od."id" AS "division_id",
    od."name" AS "division_name"
FROM "onchain_officers" of
    INNER JOIN "onchain_positions" op ON of."address" = op."officer_address"
    INNER JOIN "onchain_divisions" od ON op."division_onchain_id" = od."onchain_id"
WHERE of."address" = '0x2382488053fa5b5559d69276822fb8767e7bd546';