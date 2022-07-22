ALTER TABLE location RENAME TO location_old;
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
);
INSERT INTO location (tst, lat, lon,acc,alt,vac,batt,tid,vel,created_at)
SELECT tst, lat, lon,acc,alt,vac,batt,tid,vel,created_at
FROM location_old;
DROP TABLE location_old;