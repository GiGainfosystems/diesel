CREATE TABLE followings (
  user_id INTEGER NOT NULL,
  post_id INTEGER NOT NULL,
  email_notifications char(1) DEFAULT 'f' NOT NULL,
  constraint followings_pkey PRIMARY KEY (user_id, post_id)
);
