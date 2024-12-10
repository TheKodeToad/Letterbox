CREATE TABLE threads (
	"source_channel_id" BIGINT UNIQUE,
	"target_channel_id" BIGINT UNIQUE
);

CREATE INDEX threads_idx_by_source ON threads ("source_channel_id");
CREATE INDEX threads_idx_by_target ON threads ("target_channel_id");
