CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id        UUID               DEFAULT uuid_generate_v4() PRIMARY KEY,
    username  VARCHAR   NOT NULL UNIQUE,
    email     VARCHAR   NOT NULL UNIQUE,
--     email_verified smallint  NOT NULL default false,
    password  VARCHAR   NOT NULL,
    nick_name VARCHAR   NULL,
    avatar    VARCHAR   NULL,
    create_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    update_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);