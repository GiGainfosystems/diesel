CREATE TABLE users (
  id INT PRIMARY KEY,
  name VARCHAR2(255) NOT NULL,
  hair_color VARCHAR2(255)
);

CREATE SEQUENCE users_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1;
create or replace trigger users_id
before insert on users
for each row
when (new.id is null)
begin
select users_id_sequence.nextval
into :new.id
from dual;
end;
/

CREATE TABLE posts (
  id INT PRIMARY KEY,
  user_id INTEGER NOT NULL,
  title VARCHAR2(255) NOT NULL,
  body CLOB
);

CREATE SEQUENCE posts_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1;
create or replace trigger posts_id
before insert on posts
for each row
when (new.id is null)
begin
select posts_id_sequence.nextval
into :new.id
from dual;
end;
/

CREATE TABLE comments (
  id INT PRIMARY KEY,
  post_id INTEGER NOT NULL,
  text CLOB NOT NULL
);


CREATE SEQUENCE comments_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1;
create or replace trigger comments_id
before insert on comments
for each row
when (new.id is null)
begin
select comments_id_sequence.nextval
into :new.id
from dual;
end;
/