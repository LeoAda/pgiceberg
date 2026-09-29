FROM postgres:18
RUN apt-get update \
 && apt-get install -y --no-install-recommends postgresql-18-wal2json \
 && rm -rf /var/lib/apt/lists/*
COPY postgresql.conf /etc/postgresql/postgresql.conf
CMD ["postgres", "-c", "config_file=/etc/postgresql/postgresql.conf"]