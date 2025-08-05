CREATE TABLE modules (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    hash TEXT NOT NULL,
    compiler_version TEXT NOT NULL,
    zluda_version TEXT NOT NULL,
    device TEXT NOT NULL,
    flags BIGINT NOT NULL,
    binary BLOB NOT NULL,
    last_access BIGINT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS modules_index ON modules (hash, compiler_version, zluda_version, device, flags);

CREATE TABLE IF NOT EXISTS globals (
    key TEXT PRIMARY KEY,
    value BIGINT NOT NULL
) WITHOUT ROWID;

INSERT OR IGNORE INTO globals (key, value) VALUES ('total_size', 0);

CREATE TRIGGER IF NOT EXISTS update_size_on_delete
AFTER 
    DELETE ON modules FOR EACH ROW BEGIN 
UPDATE 
    globals 
SET 
    value = value - length(OLD.binary) 
WHERE 
    key = 'total_size';
END;

CREATE TRIGGER IF NOT EXISTS update_size_on_insert
AFTER 
    INSERT ON modules FOR EACH ROW BEGIN 
UPDATE 
    globals 
SET 
    value = value + length(NEW.binary) 
WHERE 
    key = 'total_size';
END;
