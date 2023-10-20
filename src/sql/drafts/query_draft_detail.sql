SELECT d."id",
    d."name",
    d."document_no",
    d."document_name",
    dt."name" AS document_type,
    d."updated_at",
    doc."resource_uri" as doc_uri,
    o."username" as drafter_username,
    oo."name" AS drafter_name
FROM "drafts" d
    INNER JOIN "documents" doc ON d."document_hash" = doc."hash"
    INNER JOIN "document_types" dt ON d."document_type" = dt."id"
    INNER JOIN "officers" o ON d."drafter" = o."id"
    INNER JOIN "onchain_officers" oo ON o."onchain_address" = oo."address"
WHERE d."id" = $1;