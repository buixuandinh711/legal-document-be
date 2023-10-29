SELECT of."name" as signer_name,
    op."name" as position_name
FROM "onchain_document_signatures" ods
    INNER JOIN "onchain_officers" of ON of."address" = ods."signers_address"
    INNER JOIN "onchain_positions" op on op."officer_address" = ods."signers_address"
    AND op."division_onchain_id" = ods."division_onchain_id"
    AND op."position_index" = ods."position_index"
WHERE ods."document_content_hash" = $1;