declare
begin
--execute immediate q'<CREATE OR REPLACE TYPE citext as VARCHAR2(2000)>';
execute immediate q'<CREATE TABLE citext_table (citext_field VARCHAR2(2000) PRIMARY KEY)>';

end;