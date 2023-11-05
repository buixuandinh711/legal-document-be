SELECT d."id",
    d."name"
FROM "drafts" d
    INNER JOIN "onchain_positions" op ON op."officer_address" = d."drafter_address"
    AND op."division_onchain_id" = d."division_onchain_id"
    AND op."position_index" = d."position_index"
    LEFT JOIN "onchain_documents" od ON od."document_content_hash" = d."document_hash"
WHERE d."drafter_address" = $1
    AND d."division_onchain_id" = $2
    AND d."position_index" = $3
    AND od."document_content_hash" IS NULL;