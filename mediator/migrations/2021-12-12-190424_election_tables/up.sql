-- List of all collectors in the database
CREATE TABLE collectors (
  id UUID NOT NULL PRIMARY KEY,              -- Each collector is given a unique UUID
  name VARCHAR(255) NOT NULL,                -- Also has a human-readable name
  private_base_uri TEXT NOT NULL,            -- Internal base URI (without http:// prefix and without the /api/v1 suffix)
  is_secure BOOLEAN NOT NULL DEFAULT false   -- Is a secure connection required for this collector? (https:// or wss://)
);

-- List of elections
CREATE TABLE elections (
  id UUID NOT NULL PRIMARY KEY,

  -- List of election collectors is protected information
  is_public BOOLEAN NOT NULL DEFAULT false,
  creator_id UUID NOT NULL,

  -- Cached copy of values from the API server:
  --
  -- g^x (mod p) is a cyclic group of order p-1
  --   These values are not generated until the election is closed
  generator NUMERIC NOT NULL,
  prime NUMERIC NOT NULL
);

-- Each election has one or more question
CREATE TABLE questions (
  id UUID NOT NULL PRIMARY KEY,
  election_id UUID NOT NULL REFERENCES elections(id) ON DELETE CASCADE
);

-- List of users registered in the election
CREATE TABLE registrations (
  user_id UUID NOT NULL,
  election_id UUID NOT NULL REFERENCES elections(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, election_id)
);

-- List of collectors associated with each election
CREATE TABLE election_collectors (
  election_id UUID NOT NULL REFERENCES elections(id) ON DELETE CASCADE,
  collector_id UUID NOT NULL REFERENCES collectors(id),
  PRIMARY KEY (election_id, collector_id)
);