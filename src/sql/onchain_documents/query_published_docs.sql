SELECT od."document_content_hash",
    od."number",
    od."name",
    of."name" as officer_name,
    op."name" as position_name,
    od."published_timestamp"
FROM "onchain_documents" od
    INNER JOIN "onchain_officers" of on of."address" = od."publisher_address"
    INNER JOIN "onchain_positions" op on op."officer_address" = od."publisher_address"
    AND op."division_onchain_id" = od."publisher_division_id"
    AND op."position_index" = od."publisher_position_index"
WHERE od."division_id" = $1;