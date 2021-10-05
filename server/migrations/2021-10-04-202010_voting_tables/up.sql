-- All votes occur within an election
CREATE TABLE elections (
  id UUID NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_by UUID NOT NULL REFERENCES users (id),
  status INTEGER NOT NULL,

  -- g^x (mod p) is a cyclic group of order p-1
  --   These values are not generated until the election is closed
  prime NUMERIC NOT NULL,
  generator NUMERIC NOT NULL,

  -- Used as part of the encryption for the location
  encryption_key BYTEA NOT NULL
);


-- Each election has many questions
CREATE TABLE questions (
  id UUID NOT NULL PRIMARY KEY,
  election_id UUID NOT NULL REFERENCES elections (id),
  question VARCHAR(255) NOT NULL,

  -- Cached values after electiton has ended
  final_forward_ballot NUMERIC NULL DEFAULT NULL,
  final_reverse_ballot NUMERIC NULL DEFAULT NULL,
  ballot_valid BOOLEAN NOT NULL DEFAULT false
);


-- Each question has many candidates
CREATE TABLE candidates (
  id UUID NOT NULL PRIMARY KEY,
  question_id UUID NOT NULL REFERENCES questions (id),
  candidate VARCHAR(255) NOT NULL,

  -- Cached values after election has ended
  num_votes BIGINT NULL DEFAULT NULL
);


-- Users register for an election
CREATE TABLE registrations (
  user_id UUID NOT NULL REFERENCES users (id),
  election_id UUID NOT NULL REFERENCES elections (id),
  PRIMARY KEY (user_id, election_id),

  -- User voting location
  -- Encrypted using AES keys from the server, collector 1, and collector 2
  encrypted_location BYTEA NOT NULL
);


-- Actual vote being cast
CREATE TABLE commitments (
  user_id UUID NOT NULL REFERENCES users (id),
  election_id UUID NOT NULL REFERENCES elections (id),
  question_id UUID NOT NULL REFERENCES questions (id),
  PRIMARY KEY (user_id, election_id, question_id),
  FOREIGN KEY (user_id, election_id) REFERENCES registrations (user_id, election_id),

  -- Ballots
  forward_ballot NUMERIC NOT NULL,         -- p_i
  reverse_ballot NUMERIC NOT NULL,         -- p_i'

  -- Commitments needed for verification
  g_s NUMERIC NOT NULL,         -- g^(s_ii)
  g_s_prime NUMERIC NOT NULL,   -- g^(s_ii')
  g_s_s_prime NUMERIC NOT NULL, -- g^(s_ii * s_ii')

  single_vote_verified BOOLEAN NOT NULL DEFAULT false,        -- Sub-Protocol 1
  published_ballots_verified BOOLEAN NOT NULL DEFAULT false   -- Sub-Protocol 2
);