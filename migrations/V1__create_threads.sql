CREATE TABLE "threads" (
	"id" BIGINT PRIMARY KEY,
	"dm_channel_id" BIGINT NOT NULL UNIQUE
);

CREATE INDEX "threads_idx_by_dm_channel" ON "threads" ("dm_channel_id");
