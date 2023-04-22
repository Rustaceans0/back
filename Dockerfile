FROM postgres:latest

# Set the environment variables for the PostgreSQL server
ENV POSTGRES_USER actix
ENV POSTGRES_PASSWORD actix
ENV POSTGRES_DB database

# Copy the SQL script to initialize the database
COPY init.sql /docker-entrypoint-initdb.d/

# Expose the default PostgreSQL port
EXPOSE 5432