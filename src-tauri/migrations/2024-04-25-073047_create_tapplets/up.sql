-- Your SQL goes here
-- This file should undo anything in `up.sql`
CREATE TABLE tapplet (
  id INTEGER PRIMARY KEY,
  registry_id TEXT NOT NULL,
  display_name TEXT NOT NULL,
  author_name TEXT NOT NULL,
  author_website TEXT NOT NULL,
  about_summary TEXT NOT NULL,
  about_description TEXT NOT NULL,
  category TEXT NOT NULL,
  package_name TEXT NOT NULL,
  image_id INTEGER,
  UNIQUE(package_name),
  FOREIGN KEY (image_id) REFERENCES asset(id)
);

CREATE TABLE tapplet_version (
  id INTEGER PRIMARY KEY,
  tapplet_id INTEGER,
  version TEXT NOT NULL,
  integrity TEXT NOT NULL,
  registry_url TEXT NOT NULL,
  UNIQUE(version, tapplet_id),
  FOREIGN KEY (tapplet_id) REFERENCES tapplet(id)
);

CREATE TABLE installed_tapplet (
  id INTEGER PRIMARY KEY,
  tapplet_id INTEGER,
  tapplet_version_id INTEGER,
  UNIQUE(tapplet_id, tapplet_version_id),
  FOREIGN KEY (tapplet_id) REFERENCES tapplet(id)
  FOREIGN KEY (tapplet_version_id) REFERENCES tapplet_version(id)
);

CREATE TABLE asset (
  id INTEGER PRIMARY KEY,
  rel_path TEXT NOT NULL
);

CREATE TABLE dev_tapplet (
  id INTEGER PRIMARY KEY,
  package_name TEXT NOT NULL,
  endpoint TEXT NOT NULL,
  tapplet_name TEXT NOT NULL,
  display_name TEXT NOT NULL,
  UNIQUE(endpoint)
);
