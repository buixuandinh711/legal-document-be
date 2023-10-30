SELECT d."id",
    d."name"
FROM "drafts" d
    INNER JOIN "officers" o ON d."drafter" = o."id"
    INNER JOIN "onchain_positions" op ON o."onchain_address" = op."officer_address"
    LEFT JOIN "onchain_documents" od ON od."document_content_hash" = d."document_hash"
WHERE o."id" = $1
    AND op."division_onchain_id" = $2
    AND op."position_index" = $3
    AND od."document_content_hash" IS NULL;