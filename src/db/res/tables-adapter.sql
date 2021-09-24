-- name: 1-create-users-accounts
CREATE TABLE users_accounts
(
    user_id              text PRIMARY KEY,
    account_id           varchar(50) unique,
    connected_account_id varchar(50) unique,
    mnemonic             bytea
);

-- name: 2-create-request-status
CREATE TABLE request_status
(
    id   OID,
    name varchar(20),
    CONSTRAINT pk_request_status PRIMARY KEY (id)
);

-- name: 2.1
INSERT INTO request_status(id, name)
VALUES ('0', 'Got');

-- name: 2.2
INSERT INTO request_status(id, name)
VALUES ('1', 'InProgress');

-- name: 2.3
INSERT INTO request_status(id, name)
VALUES ('2', 'Success');

-- name: 2.4
INSERT INTO request_status(id, name)
VALUES ('3', 'Error');

-- name: 2.5
INSERT INTO request_status(id, name)
VALUES ('4', 'Complete');

-- name: 2.6
INSERT INTO request_status(id, name)
VALUES ('5', 'SendError');

-- name: 3-create-requests
CREATE TABLE requests
(
    id      text,
    user_id text,
    json    jsonb,
    status  OID NOT NULL,
    hash    text,
    CONSTRAINT pk_id PRIMARY KEY (id),
    CONSTRAINT fk_account_to FOREIGN KEY (user_id)
        REFERENCES users_accounts (user_id),
    CONSTRAINT fk_status FOREIGN KEY (status)
        REFERENCES request_status (id)
);
