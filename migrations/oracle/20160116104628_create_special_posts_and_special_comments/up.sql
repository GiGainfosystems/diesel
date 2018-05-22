declare
begin
execute immediate q'<CREATE TABLE special_posts (
  id INT PRIMARY KEY,
  user_id INTEGER NOT NULL,
  title VARCHAR2(255) NOT NULL
)>';

execute immediate q'<CREATE SEQUENCE special_posts_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';
execute immediate q'<create or replace trigger special_posts_id
before insert on special_posts
for each row
when (new.id is null)
begin
select special_posts_id_sequence.nextval
into :new.id
from dual;
end>';

execute immediate q'<CREATE TABLE special_comments (
  id INT PRIMARY KEY,
  special_post_id INTEGER NOT NULL
)>';

execute immediate q'<CREATE SEQUENCE special_comments_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';

execute immediate q'<create or replace trigger special_comments_id
before insert on special_comments
for each row
when (new.id is null)
begin
select special_comments_id_sequence.nextval
into :new.id
from dual;
end>';

end;
