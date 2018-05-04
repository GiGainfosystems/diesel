CREATE OR REPLACE TYPE vcarray AS VARRAY(255) OF VARCHAR2(255);
/
ALTER TABLE posts ADD (tags vcarray default vcarray() NOT NULL);
