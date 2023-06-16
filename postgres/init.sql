-- Every command ever invoked is added here, currently only supports on_command_completion (not app commands)
CREATE TABLE IF NOT EXISTS commands
(
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    guild_id   BIGINT,
    channel_id BIGINT,
    author_id  BIGINT,
    used       TIMESTAMP,
    prefix     TEXT,
    command    TEXT,
    slash      BOOLEAN,
    failed     BOOLEAN
);

CREATE TABLE IF NOT EXISTS message_metrics
(
    msg_id      BIGINT    NOT NULL PRIMARY KEY,
    msg_created TIMESTAMP NOT NULL,
    user_id     BIGINT    NOT NULL,
    channel_id  BIGINT    NOT NULL,
    guild_id    BIGINT    NOT NULL
);

CREATE TABLE IF NOT EXISTS vc_metrics
(
    id         BIGINT    GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id    BIGINT    NOT NULL,
    channel_id BIGINT    NOT NULL,
    guild_id   BIGINT    NOT NULL,
    -- In seconds
    time_spent INT       NOT NULL,
    -- Time the user leaves (ends the session)
    -- for a potential leaderboard resetting timer
    time_left  TIMESTAMP NOT NULL
);

CREATE TYPE bal_type AS ENUM ('wallet', 'bank');

CREATE TABLE IF NOT EXISTS economy
(
    id         BIGINT      GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id    BIGINT      NOT NULL,
    guild_id   BIGINT      NOT NULL,
    coins      BIGINT      NOT NULL,
    bal_type   bal_type    NOT NULL,
    time       TIMESTAMP   NOT NULL,
    -- The message/VC session that the coins came from
    msg        BIGINT      REFERENCES message_metrics (msg_id),
    vc_session BIGINT      REFERENCES vc_metrics (id),
    -- Represent whether or not the source of income is from -daily/weekly/monthly
    daily      BOOLEAN     NOT NULL,
    weekly     BOOLEAN     NOT NULL,
    monthly    BOOLEAN     NOT NULL
);
