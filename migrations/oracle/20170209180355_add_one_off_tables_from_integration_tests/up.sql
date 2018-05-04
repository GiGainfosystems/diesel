CREATE TABLE numbers (n INTEGER PRIMARY KEY);
CREATE TABLE precision_numbers (n DOUBLE PRECISION PRIMARY KEY NOT NULL);
CREATE TABLE nullable_doubles (id INT PRIMARY KEY, n DOUBLE PRECISION);
CREATE SEQUENCE nullable_doubles_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1;
create or replace trigger nullable_doubles_id
before insert on nullable_doubles
for each row
when (new.id is null)
begin
select nullable_doubles_id_sequence.nextval
into :new.id
from dual;
end;
/
CREATE TABLE users_with_name_pk (name VARCHAR2(255) PRIMARY KEY);
CREATE TABLE points (x INTEGER NOT NULL, y INTEGER NOT NULL, constraint points_pkey PRIMARY KEY (x, y));
