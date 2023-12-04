SELECT d."id",
    d."name",
    d."document_no",
    d."document_name",
    dt."name" AS document_type,
    d."file_name",
    d."updated_at",
    doc."resource_uri" as doc_uri,
    oo."name" AS drafter_name,
    op."name" AS drafter_pos
FROM "drafts" d
    INNER JOIN "documents" doc ON d."document_hash" = doc."hash"
    INNER JOIN "document_types" dt ON d."document_type" = dt."id"
    INNER JOIN "onchain_officers" oo ON d."drafter_address" = oo."address"
    INNER JOIN "onchain_positions" op ON op."officer_address" = d."drafter_address"
    AND op."division_onchain_id" = d."division_onchain_id"
    AND op."position_index" = d."position_index"
WHERE d."id" = $1;