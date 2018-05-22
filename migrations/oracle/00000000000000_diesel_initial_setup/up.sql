-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.
declare
begin
execute immediate q'<create or replace
function quote_ident
    (
        ident in varchar2
    ) return varchar2 as
    tmp varchar2(32000);
pos integer;
begin
pos := INSTR(ident, '.');
IF pos = 0 THEN
tmp := '"' || ident || '"';
ELSE
tmp := '"' || SUBSTR(ident,1,pos-1) || '"."' || SUBSTR(ident,pos+1) || '"';
END IF;
return tmp;
end quote_ident;>';

execute immediate q'<
create or replace
function quote_literal
    (
        ident in varchar2
    ) return varchar2 as
    tmp varchar2(32000);
begin
tmp := '''' || ident || '''';
return tmp;
end quote_literal;>';

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id int PRIMARY KEY, updated_at TIMESTAMP DEFAULT sysdate not null);
--
-- declare
-- begin
--    diesel_manage_updated_at('users');
-- end;
-- ```
execute immediate q'<
CREATE OR REPLACE PROCEDURE diesel_manage_updated_at(tbl in varchar2)
AS
BEGIN
EXECUTE  immediate 'create or replace trigger set_updated_at_' || quote_ident(tbl) ||'
before update on '|| quote_ident(tbl) ||'
for each row
when (new.updated_at is null)
begin
select sysdate
into :new.updated_at
from dual;
end;
/';
END diesel_manage_updated_at;
>';

end;


