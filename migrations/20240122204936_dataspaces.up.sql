create table dataspaces (
    id varchar(21) primary key,
    name text not null
);

create table dataspace_memebers (
    dataspace_id varchar(21) not null references dataspaces(id),
    user_id varchar(21) not null references users(id)
);
