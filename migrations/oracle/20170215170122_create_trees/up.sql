begin
execute immediate q'<CREATE TABLE trees (
  id INT PRIMARY KEY,
  parent_id INTEGER
)>';
execute immediate q'<CREATE SEQUENCE trees_id_sequence
  INCREMENT BY 1
  MINVALUE 1
  MAXVALUE 9223372036854775807
  START WITH 1>';
execute immediate q'<create or replace trigger trees_id
before insert on trees
for each row
when (new.id is null)
begin
select trees_id_sequence.nextval
into :new.id
from dual;
end>';

end;
