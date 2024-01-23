create table dataspace_data_warehouses (
  dataspace_id varchar(21) primary key references dataspaces(id),
  username text not null,
  password_hashed text not null,
  database text not null
)
