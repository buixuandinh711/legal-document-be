SELECT DISTINCT d."id",
    d."name",
    d."updated_at",
    oo."name" AS drafter_name,
    op."name" AS drafter_pos,
    CASE
        WHEN od."document_content_hash" IS NULL THEN 0
        ELSE 1
    END AS "status"
FROM "drafts" d
    INNER JOIN "onchain_positions" op ON op."officer_address" = d."drafter_address"
    AND op."division_onchain_id" = d."division_onchain_id"
    AND op."position_index" = d."position_index"
    INNER JOIN "onchain_officers" oo ON oo."address" = d."drafter_address"
    LEFT JOIN "onchain_documents" od ON od."document_content_hash" = d."document_hash"
WHERE (
        d."drafter_address" = $1
        AND d."division_onchain_id" = $2
        AND d."position_index" = $3
    )
    OR d."id" IN (
        SELECT DISTINCT dt."draft_id"
        FROM "draft_tasks" dt
        WHERE dt."id" IS NOT NULL
            AND dt."assigner_address" = $1
            AND dt."assigner_division_id" = $2
            AND dt."assigner_position_index" = $3
    );