CREATE TABLE "threads" (
	"id" BIGINT PRIMARY KEY,
	"dm_channel_id" BIGINT NOT NULL UNIQUE,
	"user_id" BIGINT NOT NULL UNIQUE,
	"opened_by_id" BIGINT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL
);

CREATE INDEX "threads_idx_by_dm_channel" ON "threads" ("dm_channel_id");
CREATE INDEX "threads_idx_by_user" ON "threads" ("user_id");
