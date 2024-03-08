-- Add migration script here
-- Create subscriptions table
CREATE TABLE subscriptions(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  sunbscribed_at timestamptz NOT NULL
); 
