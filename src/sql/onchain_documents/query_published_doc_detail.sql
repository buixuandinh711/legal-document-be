SELECT od."document_content_hash",
    od."number",
    od."name",
    od."doc_type",
    od."published_timestamp",
    of."name" as officer_name,
    op."name" as position_name,
    doc."resource_uri"
FROM "onchain_documents" od
    INNER JOIN "onchain_officers" of on of."address" = od."publisher_address"
    INNER JOIN "onchain_positions" op on op."officer_address" = od."publisher_address"
    AND op."division_onchain_id" = od."publisher_division_id"
    AND op."position_index" = od."publisher_position_index"
    INNER JOIN "documents" doc ON doc."hash" = od."document_content_hash"
WHERE od."document_content_hash" = $1;