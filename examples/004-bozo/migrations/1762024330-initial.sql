-- migrate:up

DO $$
BEGIN
	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'address'
	) THEN
		CREATE DOMAIN ADDRESS AS CHAR(42);
	END IF;

	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'hash'
	) THEN
		CREATE DOMAIN HASH AS CHAR(66);
	END IF;
END $$;

CREATE TALBE bozo_comments_1 (
	id SERIAL PRIMARY KEY,
	created_by TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	content VARCHAR NOT NULL,
	wallet ADDRESS NOT NULL,
	transaction_hash HASH NOT NULL
);

CREATE INDEX ON bozo_comments_1 (transaction_hash);

CREATE INDEX ON bozo_comments_1 (wallet);

-- migrate:down
