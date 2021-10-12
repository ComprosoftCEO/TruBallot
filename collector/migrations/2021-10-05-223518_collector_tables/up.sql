-- List of registered elections
CREATE TABLE elections (
  id UUID NOT NULL PRIMARY KEY,

  -- Cached copy of values from the API server:
  --
  -- g^x (mod p) is a cyclic group of order p-1
  --   These values are not generated until the election is closed
  generator NUMERIC NOT NULL,
  prime NUMERIC NOT NULL,

  -- Private key used by Paillier Cryptosystem for secure, two-party multiplication (STPM)
  paillier_p NUMERIC NOT NULL,
  paillier_q NUMERIC NOT NULL,

  -- Used as part of the encryption for the location
  encryption_key BYTEA NOT NULL
);


-- Each election has many questions
CREATE TABLE questions (
  id UUID NOT NULL PRIMARY KEY,
  election_id UUID NOT NULL REFERENCES elections (id),
  num_candidates BIGINT NOT NULL
);


-- Generated private values used for verification
CREATE TABLE registrations (
  user_id UUID NOT NULL,
  election_id UUID NOT NULL REFERENCES elections (id),
  question_id UUID NOT NULL REFERENCES questions (id),
  PRIMARY KEY (user_id, election_id, question_id),

  -- Rows of the shares matrix
  forward_verification_shares NUMERIC NOT NULL,  -- S_c,i
  reverse_verification_shares NUMERIC NOT NULL,  -- S_c,i'

  -- Columns of the shares matrix
  forward_ballot_shares NUMERIC NOT NULL,  -- S~c,i
  reverse_ballot_shares NUMERIC NOT NULL   -- S~c,i'
);