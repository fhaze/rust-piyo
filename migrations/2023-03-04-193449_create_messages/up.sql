-- Your SQL goes here
create table messages (
  id integer not null primary key autoincrement,
  msg varchar not null,
  user_id integer not null,
  foreign key(user_id) references users(id)
);
