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

--
-- Economy
--

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

--
-- Inventory
--
CREATE TABLE IF NOT EXISTS explore_items
(
    id         SMALLINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name       TEXT     NOT NULL,
    sell_price BIGINT   NOT NULL,
    tier       TEXT     NOT NULL, -- e.g. legendary
    -- reward upon consumption, if any
    health     SMALLINT,
    emoji_id   BIGINT   NOT NULL,
    emoji_name TEXT     NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory
(
    id       BIGINT    GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id  BIGINT    NOT NULL,
    guild_id BIGINT    NOT NULL,
    UNIQUE (user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS inventory_items  -- map inventory and items
(
    item_id      SMALLINT  NOT NULL REFERENCES explore_items(id),
    inventory_id INT       NOT NULL REFERENCES inventory(id),
    time         TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS profiles
(
    user_id     BIGINT,
    guild_id    BIGINT,
    health      SMALLINT NOT NULL DEFAULT 100,
    health_lvl  SMALLINT NOT NULL DEFAULT 0,
    regen_lvl   SMALLINT NOT NULL DEFAULT 0,
    defence_lvl SMALLINT NOT NULL DEFAULT 0,
    attack_lvl  SMALLINT NOT NULL DEFAULT 0,
    weapon      SMALLINT REFERENCES explore_items(id),
    armour      SMALLINT REFERENCES explore_items(id),
    PRIMARY KEY (user_id, guild_id)
);
