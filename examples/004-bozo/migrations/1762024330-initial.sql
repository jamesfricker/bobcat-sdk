-- migrate:up

DO $$
BEGIN
	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'hash'
	) THEN
		CREATE DOMAIN HASH AS CHAR(66);
	END IF;
END $$;

CREATE TABLE bozo_comments_1 (
	id SERIAL PRIMARY KEY,
	created_by TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	epoch INTEGER NOT NULL,
	epoch_seq INTEGER NOT NULL,
	content VARCHAR NOT NULL,
	transaction_hash HASH NOT NULL
);

CREATE INDEX ON bozo_comments_1 (epoch);

CREATE UNIQUE INDEX ON bozo_comments_1 (epoch, epoch_seq);

CREATE FUNCTION bozo_insert_comment_1(
	p_epoch INTEGER,
	p_content VARCHAR,
	p_transaction_hash HASH
)
RETURNS VOID
AS $$
DECLARE
	v_epoch_seq INTEGER;
BEGIN
	SELECT COALESCE(MAX(epoch_seq), 0) + 1
	INTO v_epoch_seq
	FROM bozo_comments_1
	WHERE epoch = p_epoch;

	INSERT INTO bozo_comments_1 (epoch, epoch_seq, content, transaction_hash)
	VALUES (p_epoch, v_epoch_seq, p_content, p_transaction_hash);
END;
$$ LANGUAGE plpgsql;

-- migrate:down
