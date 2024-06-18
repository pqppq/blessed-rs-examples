CREATE DATABASE example;
CREATE USER john WITH PASSWORD 'password';

\c example;
CREATE TABLE users (
		id    INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email TEXT   NOT NULL,
    name  TEXT   NOT NULL,
    age   SMALLINT
);

GRANT SELECT ON ALL TABLES IN SCHEMA public TO john;

INSERT INTO users (email, name, age) VALUES
('john@example.com'   ,'john',   24 ),
('kevin@example.com'  ,'kevin',  18 ),
('michel@example.com' ,'michel', 32 ),
('bob@example.com'    ,'bob',    43 ),
('kate@example.com'   ,'kate',   51 );
