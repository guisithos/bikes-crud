#!/bin/bash
set -e
export PGPASSWORD=$POSTGRES_PASSWORD;
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE USER $APP_DB_USER WITH PASSWORD '$APP_DB_PASS';
  CREATE DATABASE $APP_DB_NAME;
  GRANT ALL PRIVILEGES ON DATABASE $APP_DB_NAME TO $APP_DB_USER;
  \connect $APP_DB_NAME $APP_DB_USER
  BEGIN;
  CREATE TABLE IF NOT EXISTS bikes (
    id SERIAL PRIMARY KEY,
    modelo varchar(250) NOT NULL,
    cor varchar(250) NOT NULL,
    cilindradra varchar(250) NOT NULL
  );
   
  INSERT INTO bikes(modelo, cor, cilindrada) VALUES ('Ducati Panigale V4 S', 'Vermelha', '1103');

  INSERT INTO bikes(modelo, cor, cilindrada) VALUES ('Harley Davidson Iron 883', 'Cinza', '883');

  INSERT INTO bikes(modelo, cor, cilindrada) VALUES ('Suzuki GSX 750 SRAD', 'Azul', '750');

  SELECT * FROM bikes;
  
  COMMIT;
EOSQL
