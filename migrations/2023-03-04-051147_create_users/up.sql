-- Your SQL goes here
create table users (
  id integer not null primary key autoincrement,
  name varchar not null,
  email varchar not null,
  constraint unique_email unique (email)
);
