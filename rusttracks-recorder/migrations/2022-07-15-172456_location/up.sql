CREATE TABLE IF NOT EXISTS location (
    tst BIGINT NOT NULL,
    lat DECIMAL(8, 6) NOT NULL,
    lon DECIMAL(9,6) NOT NULL,
    acc INTEGER,
    alt INTEGER,
    vac INTEGER,
    batt INTEGER,
    tid TEXT NOT NULL,
    vel INTEGER,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (tst, tid)
)