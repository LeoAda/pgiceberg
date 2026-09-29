CREATE TABLE demo (id integer PRIMARY KEY);
ALTER TABLE demo REPLICA IDENTITY FULL; 
insert into demo values (1),(2);
select * from demo;

SELECT pg_create_logical_replication_slot(
    'datalake_slot',      -- Slot name
    'pgoutput'      -- Output plugin
);
SELECT
    slot_name,
    plugin,
    slot_type,
    active,
    restart_lsn,
    confirmed_flush_lsn
FROM pg_replication_slots;
SELECT
    slot_name,
    active,
    pg_size_pretty(
        pg_wal_lsn_diff(
            pg_current_wal_lsn(),
            restart_lsn
        )
    ) AS lag_size
FROM pg_replication_slots;

CREATE PUBLICATION datalake_pub FOR TABLE
    demo;
SELECT * FROM pg_publication;
SELECT * FROM pg_publication_tables
WHERE pubname = 'datalake_pub';

-- Peek at changes without consuming them
-- Useful for testing and debugging

UPDATE demo SET id = id;
SELECT * FROM pg_logical_slot_peek_binary_changes(
    'datalake_slot',     -- Slot name
    NULL,          -- Start LSN (NULL = from last position)
    NULL,          -- Number of changes (NULL = all available)
    'proto_version', '1',
    'publication_names', 'datalake_pub'
);

-- or add a primary key

SELECT * FROM pg_logical_slot_get_binary_changes(
    'datalake_slot',
    NULL,
    NULL,
    'proto_version', '1',
    'publication_names', 'datalake_pub'
);