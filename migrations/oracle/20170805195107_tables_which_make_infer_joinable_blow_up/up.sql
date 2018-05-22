declare
begin

execute immediate q'<CREATE TABLE self_referential_fk (
  id INT PRIMARY KEY,
  parent_id INTEGER NOT NULL
)>';

execute immediate q'<CREATE SEQUENCE self_referential_fk_id_seq
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger self_referential_fk_id
before insert on self_referential_fk
for each row
when (new.id is null)
begin
select self_referential_fk_id_seq.nextval
into :new.id
from dual;
end>';

execute immediate q'<ALTER TABLE self_referential_fk ADD CONSTRAINT self_referential_fk_parent_id_fk FOREIGN KEY (parent_id) REFERENCES self_referential_fk>';

execute immediate q'<ALTER TABLE posts ADD CONSTRAINT title_is_unique UNIQUE (title)>';
execute immediate q'<CREATE TABLE fk_doesnt_reference_pk (
  id INT PRIMARY KEY,
  random TEXT REFERENCES posts (title)
)>';

execute immediate q'<CREATE SEQUENCE fk_doesnt_reference_pk_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger fk_doesnt_reference_pk_id
before insert on fk_doesnt_reference_pk
for each row
when (new.id is null)
begin
select fk_doesnt_reference_pk_id_sequence.nextval
into :new.id
from dual;
end>';


execute immediate q'<CREATE TABLE composite_fk (
  id INT PRIMARY KEY,
  post_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id, post_id) REFERENCES followings
)>';

execute immediate q'<CREATE SEQUENCE composite_fk_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger composite_fk_id
before insert on composite_fk
for each row
when (new.id is null)
begin
select composite_fk_id_sequence.nextval
into :new.id
from dual;
end>';

execute immediate q'<CREATE TABLE multiple_fks_to_same_table (
  id INT PRIMARY KEY,
  post_id_1 INTEGER REFERENCES posts,
  post_id_2 INTEGER REFERENCES posts
)>';

execute immediate q'<CREATE SEQUENCE multiple_fks_to_same_table_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger multiple_fks_to_same_table_id
before insert on multiple_fks_to_same_table
for each row
when (new.id is null)
begin
select multiple_fks_to_same_table_id_sequence.nextval
into :new.id
from dual;
end>';

execute immediate q'<CREATE TABLE cyclic_fk_1 (
  id SERIAL PRIMARY KEY,
  cyclic_fk_2_id INTEGER
)>';

execute immediate q'<CREATE SEQUENCE cyclic_fk_1_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger cyclic_fk_1_id
before insert on cyclic_fk_1
for each row
when (new.id is null)
begin
select cyclic_fk_1_id_sequence.nextval
into :new.id
from dual;
end>';

execute immediate q'<CREATE TABLE cyclic_fk_2 (
  id SERIAL PRIMARY KEY,
  cyclic_fk_1_id INTEGER REFERENCES cyclic_fk_1
)>';

execute immediate q'<CREATE SEQUENCE cyclic_fk_2_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger cyclic_fk_2_id
before insert on cyclic_fk_2
for each row
when (new.id is null)
begin
select cyclic_fk_2_id_sequence.nextval
into :new.id
from dual;
end>';

execute immediate q'<ALTER TABLE cyclic_fk_1 ADD CONSTRAINT cyclic_fk_1_cyclic_fk_2_id_fk FOREIGN KEY (cyclic_fk_2_id) REFERENCES cyclic_fk_2>';

end;