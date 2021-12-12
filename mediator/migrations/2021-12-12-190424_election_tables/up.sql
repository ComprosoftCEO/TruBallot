-- List of all collectors in the database
CREATE TABLE collectors (
  id UUID NOT NULL PRIMARY KEY,              -- Each collector is given a unique UUID
  name VARCHAR(255) NOT NULL,                -- Also has a human-readable name
  private_base_url TEXT NOT NULL,            -- Internal base URL (without http:// prefix and without the /api/v1 suffix)
  is_secure BOOLEAN NOT NULL DEFAULT false   -- Is a secure connection required for this collector? (https:// or wss://)
);

-- List of elections
CREATE TABLE elections (
  id UUID NOT NULL PRIMARY KEY,

  -- List of election collectors is protected information
  is_public BOOLEAN NOT NULL DEFAULT false
);

-- Each election has one or more question
CREATE TABLE questions (
  id UUID NOT NULL PRIMARY KEY,
  election_id UUID NOT NULL REFERENCES elections(id)
);

-- List of users registered in the election
CREATE TABLE registrations (
  user_id UUID NOT NULL,
  election_id UUID NOT NULL REFERENCES elections(id),
  PRIMARY KEY (user_id, election_id)
);

-- List of collectors associated with each election
CREATE TABLE election_collectors (
  election_id UUID NOT NULL REFERENCES elections(id),
  collector_id UUID NOT NULL REFERENCES collectors(id),
  PRIMARY KEY (election_id, collector_id)
);