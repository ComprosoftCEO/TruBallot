-- All votes occur within an election
CREATE TABLE elections (
  id UUID NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_by UUID NOT NULL REFERENCES users (id),
  status INTEGER NOT NULL,

  is_public BOOLEAN NOT NULL DEFAULT false,
  access_code VARCHAR(6) NULL DEFAULT NULL UNIQUE,

  -- g^x (mod p) is a cyclic group of order p-1
  --   These values are not generated until the election is closed
  generator NUMERIC NOT NULL,
  prime NUMERIC NOT NULL,

  -- Modulus to use for the encrypted location from the collectors
  location_modulus NUMERIC NOT NULL
);


-- Each election has many questions
CREATE TABLE questions (
  id UUID NOT NULL PRIMARY KEY,
  election_id UUID NOT NULL REFERENCES elections (id) ON DELETE CASCADE,
  question VARCHAR(255) NOT NULL,
  question_number BIGINT NOT NULL CHECK (question_number >= 0),
  UNIQUE (election_id, question_number),

  -- Store these values from the collectors after the election has ended
  --   Set to 0 by default before then
  forward_cancelation_shares NUMERIC NOT NULL,
  reverse_cancelation_shares NUMERIC NOT NULL
);


-- Each question has many candidates
CREATE TABLE candidates (
  id UUID NOT NULL PRIMARY KEY,
  question_id UUID NOT NULL REFERENCES questions (id) ON DELETE CASCADE,
  candidate VARCHAR(255) NOT NULL,
  candidate_number BIGINT NOT NULL CHECK (candidate_number >= 0),
  UNIQUE (question_id, candidate_number)
);


-- Users register for an election
CREATE TABLE registrations (
  user_id UUID NOT NULL REFERENCES users (id),
  election_id UUID NOT NULL REFERENCES elections (id),
  PRIMARY KEY (user_id, election_id)
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