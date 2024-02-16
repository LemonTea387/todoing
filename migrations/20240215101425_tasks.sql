CREATE TABLE IF NOT EXISTS tasks 
(
  id          INTEGER PRIMARY KEY NOT NULL,
  title       TEXT NOT NULL,
  description TEXT,
  priority    INTEGER,
  created_dt  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tasks_activity 
(
  activity_id INTEGER PRIMARY KEY,
  status      INTEGER NOT NULL,
  started_dt  DATETIME,
  ended_dt    DATETIME,
  task_id     INTEGER NOT NULL,
  FOREIGN KEY(task_id) REFERENCES tasks(id)
);
