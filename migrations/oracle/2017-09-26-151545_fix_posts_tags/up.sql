declare
begin
execute immediate q'<CREATE OR REPLACE TYPE vclobarray AS VARRAY(255) OF VARCHAR2(32000)>';
execute immediate q'<ALTER TABLE posts drop column tags>';
execute immediate q'<ALTER TABLE posts ADD (tags vclobarray default vclobarray() NOT NULL)>';

end;

