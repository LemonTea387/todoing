CREATE TABLE IF NOT EXISTS tasks 
(
  id          INTEGER PRIMARY KEY NOT NULL,
  title       TEXT NOT NULL,
  description TEXT,
  status      BLOB NOT NULL,
  priority    BLOB,
  created_dt  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
