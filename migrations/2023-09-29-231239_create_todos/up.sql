CREATE TABLE OrgStat (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    count INT NOT NULL,
    processdate TIMESTAMP NOT NULL
)
