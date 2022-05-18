-- Add migration script here
CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

create table users
(
    id             uuid               default uuid_generate_v4() primary key,
    username       varchar   not null unique,
    email          varchar   not null unique,
--     email_verified smallint  NOT NULL default false,
    password       varchar   not null,
    nick_name      varchar null,
    avatar         varchar null,
    create_at      timestamp not null default current_timestamp,
    update_at      timestamp not null default current_timestamp
)