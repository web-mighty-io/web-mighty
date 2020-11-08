CREATE TABLE records (
    record_path TEXT NOT NULL
) 

CREATE INDEX idx 
ON records (record_path);