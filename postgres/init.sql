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
