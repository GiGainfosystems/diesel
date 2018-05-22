declare
begin

execute immediate q'<CREATE TABLE nullable_table (
    id INT PRIMARY KEY,
    value INTEGER
)>';
execute immediate q'<CREATE SEQUENCE nullable_table_id_sequence
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START WITH 1>';
execute immediate q'<create or replace trigger nullable_table_id
before insert on nullable_table
for each row
when (new.id is null)
begin
select nullable_table_id_sequence.nextval
into :new.id
from dual;
end>';

end;
