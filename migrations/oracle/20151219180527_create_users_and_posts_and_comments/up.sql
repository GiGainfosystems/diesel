declare
begin
execute immediate q'<CREATE TABLE users (
  id INT PRIMARY KEY,
  name VARCHAR2(255) NOT NULL,
  hair_color VARCHAR2(255)
)>';

execute immediate q'<CREATE SEQUENCE users_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';

execute immediate q'<create or replace trigger users_id
before insert on users
for each row
when (new.id is null)
begin
select users_id_sequence.nextval
into :new.id
from dual;
end users_id;>';

execute immediate q'<CREATE TABLE posts (
  id INT PRIMARY KEY,
  user_id INTEGER NOT NULL,
  title VARCHAR2(255) NOT NULL,
  body CLOB
)>';

execute immediate q'<CREATE SEQUENCE posts_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';

execute immediate q'<create or replace trigger posts_id
before insert on posts
for each row
when (new.id is null)
begin
select posts_id_sequence.nextval
into :new.id
from dual;
end;>';

execute immediate q'<CREATE TABLE comments (
  id INT PRIMARY KEY,
  post_id INTEGER NOT NULL,
  text CLOB NOT NULL
)>';

execute immediate q'<CREATE SEQUENCE comments_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';

execute immediate q'<create or replace trigger comments_id
before insert on comments
for each row
when (new.id is null)
begin
select comments_id_sequence.nextval
into :new.id
from dual;
end;>';
end;