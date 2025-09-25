CREATE TABLE user_roles (
  name TEXT NOT NULL PRIMARY KEY
);

INSERT INTO USER_ROLES(name) VALUES ('admin'), ('user');

CREATE TABLE USERS (
  id TEXT NOT NULL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  password TEXT NOT NULL,
  totp_secret TEXT,
  role TEXT NOT NULL DEFAULT 'user' REFERENCES USER_ROLES(name)
);

CREATE TABLE IF NOT EXISTS TRUSTED_IPS (
  user_id TEXT NOT NULL REFERENCES USERS(id),
  ip TEXT NOT NULL,
  PRIMARY KEY(user_id, ip)
);

CREATE TABLE publiek_privaat_types (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE companies (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE PERMISSIONS(
  user TEXT NOT NULL REFERENCES USERS(id) ON DELETE CASCADE,
  company TEXT REFERENCES COMPANIES(name) ON DELETE CASCADE,
  `create` BOOLEAN NOT NULL DEFAULT 0,
  edit BOOLEAN NOT NULL DEFAULT 0,
  PRIMARY KEY(user, company)
);

CREATE UNIQUE INDEX `permissions_unique_NULL_company` ON PERMISSIONS (
    user,
    IFNULL(company, '')
);


CREATE TABLE projects (
  id TEXT NOT NULL,
  name TEXT NOT NULL,
  slug TEXT GENERATED always AS (
    lower(
      replace (
        replace (
          replace (
            replace (
              replace (replace (name, ' ', '-'), '!', ''),
              '?',
              ''
            ),
            ',',
            ''
          ),
          '.',
          ''
        ),
        char(10),
        '-'
      )
    )
  ) stored NOT NULL,
  LOCATION TEXT,
  YEAR INTEGER,
  learn_more TEXT,
  status BOOLEAN NOT NULL DEFAULT 0,
  publiek_privaat TEXT NOT NULL DEFAULT 'publiek' REFERENCES publiek_privaat_types (name),
  draft BOOLEAN NOT NULL DEFAULT 0,
  unique (slug, draft),
  PRIMARY KEY (id, draft)
);

CREATE TABLE tags (
  name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE project_tags (
  project_id TEXT NOT NULL,
  tag TEXT NOT NULL REFERENCES TAGS(name),
  draft BOOLEAN NOT NULL DEFAULT 0,
  PRIMARY KEY(project_id, tag, draft),
  FOREIGN KEY(project_id, draft) REFERENCES PROJECTS(id, draft) ON DELETE CASCADE
);

CREATE TABLE project_companies (
  project_id TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  company_name TEXT REFERENCES companies (name),
  show_in_carousel BOOLEAN NOT NULL DEFAULT 0,
  introduction TEXT,
  thumbnail TEXT,
  header_photo TEXT,
  header_photo_copyright TEXT,
  banner_photo TEXT,
  banner_photo_copyright TEXT,
  visible BOOLEAN NOT NULL DEFAULT 1,
  weight INTEGER NOT NULL DEFAULT 50 check (
    weight >= 0
    AND weight <= 100
  ),
  custom_lightbox BOOLEAN DEFAULT 0,
  custom_content BOOLEAN DEFAULT 0,
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  PRIMARY KEY (project_id, draft, company_name)
);

CREATE TABLE core_numbers (
  id TEXT NOT NULL,
  project_id TEXT NOT NULL,
  title TEXT NOT NULL,
  number TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  PRIMARY KEY (id, draft),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE
);

CREATE TABLE party_types (name TEXT PRIMARY KEY NOT NULL);

CREATE TABLE parties (name TEXT PRIMARY KEY NOT NULL, url TEXT);

CREATE TABLE tms (name TEXT PRIMARY KEY NOT NULL);

CREATE TABLE tm_companies (
  name TEXT REFERENCES tms (name),
  company TEXT REFERENCES companies (name),
  PRIMARY KEY (name, company)
);

CREATE TABLE tm_party (
  name TEXT REFERENCES tms (name),
  party TEXT REFERENCES parties (name),
  PRIMARY KEY (name, party)
);

CREATE TABLE project_parties (
  project_id TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  party_name TEXT NOT NULL,
  TYPE TEXT NOT NULL REFERENCES party_types (name),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  PRIMARY KEY (project_id, draft, party_name, TYPE)
);

CREATE TABLE project_tms (
  project_id TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  tm_name TEXT NOT NULL REFERENCES tms (name),
  TYPE TEXT NOT NULL REFERENCES party_types (name),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  PRIMARY KEY (project_id, draft, tm_name, TYPE)
);

CREATE TABLE industries (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE project_industries (
  project_id TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  industry TEXT NOT NULL REFERENCES industries (name),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  PRIMARY KEY (project_id, draft, industry)
);

CREATE TABLE project_content (
  project_id TEXT NOT NULL,
  company_name TEXT,
  id TEXT NOT NULL,
  previous_entry TEXT,
  draft BOOLEAN NOT NULL DEFAULT 0,
  text TEXT NOT NULL,
  image TEXT,
  image_copyright TEXT,
  quote TEXT,
  quote_small TEXT,
  PRIMARY KEY (id, draft),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  FOREIGN key (project_id, company_name, draft) REFERENCES project_companies (project_id, company_name, draft)
);

CREATE TABLE images (
  project_id TEXT NOT NULL,
  company_name TEXT,
  id TEXT NOT NULL,
  draft BOOLEAN NOT NULL DEFAULT 0,
  image TEXT,
  image_copyright TEXT,
  alt TEXT,
  PRIMARY KEY (id, draft),
  FOREIGN key (project_id, draft) REFERENCES projects (id, draft) ON DELETE CASCADE,
  FOREIGN key (project_id, company_name, draft) REFERENCES project_companies (project_id, company_name, draft)
);

CREATE TABLE cookie_consent (
  ip TEXT NOT NULL PRIMARY KEY,
  date_modified INTEGER NOT NULL,
  analytics BOOLEAN NOT NULL
);

CREATE TABLE form_submissions (
  id TEXT NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email tEXT NOT NULL,
  phone TEXT NOT NULL,
  message TEXT NOT NULL,
  company TEXT NOT NULL REFERENCES companies (name),
  datetime DATETIME NOT NULL DEFAULT(datetime('subsec')),
  sent_to TEXT,
  recaptcha_score INTEGER
);
