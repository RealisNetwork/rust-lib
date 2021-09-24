CREATE TABLE extrinsics_batch
(
    request_id   TEXT PRIMARY KEY,
    block_number OID,
    hash         TEXT
);

CREATE TABLE extrinsics_on_chain
(
    block_number OID,
    hash TEXT PRIMARY KEY
);
