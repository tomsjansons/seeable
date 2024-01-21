-- SQLite syntax

drop table if exists sessions;
drop table if exists login_links;
drop table if exists users;

create table users (
    id varchar(21) primary key not null,
    email varchar(255) unique not null
);

create table login_links (
    id varchar(21) primary key not null,
    email varchar(255) not null,
    expires_at timestamp not null
);

create table sessions (
    id varchar(21) primary key not null,
    user_id varchar(21) not null references users(id),
    expires_at timestamp not null
);
