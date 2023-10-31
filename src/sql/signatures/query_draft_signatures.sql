SELECT of."name" as signer_name,
    op."name" as position_name,
    ds."signature"
FROM "draft_signatures" ds
    INNER JOIN "onchain_officers" of ON of."address" = ds."signer_address"
    INNER JOIN "onchain_positions" op on op."officer_address" = ds."signer_address"
    AND op."division_onchain_id" = ds."division_onchain_id"
    AND op."position_index" = ds."position_index"
WHERE ds."draft_id" = $1;