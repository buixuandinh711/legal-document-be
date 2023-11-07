SELECT DISTINCT d."id",
    d."name"
FROM "drafts" d
    INNER JOIN "onchain_positions" op ON op."officer_address" = d."drafter_address"
    AND op."division_onchain_id" = d."division_onchain_id"
    AND op."position_index" = d."position_index"
    LEFT JOIN "onchain_documents" od ON od."document_content_hash" = d."document_hash"
WHERE od."document_content_hash" IS NULL
    AND (
        (
            d."drafter_address" = '0x2382488053fa5b5559d69276822fb8767e7bd546'
            AND d."division_onchain_id" = 'H1'
            AND d."position_index" = 0
        )
        OR d."id" IN (
            SELECT DISTINCT dt."id"
            FROM "draft_tasks" dt
            WHERE dt."id" IS NOT NULL
                AND dt."assigner_address" = '0x2382488053fa5b5559d69276822fb8767e7bd546'
                AND dt."assigner_division_id" = 'H1'
                AND dt."assinger_position_index" = 0
        )
    );