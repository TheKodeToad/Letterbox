CREATE TABLE "received_messages" (
	"id" BIGINT PRIMARY KEY,
	"thread_id" BIGINT NOT NULL REFERENCES "threads" ON DELETE CASCADE,
	"forwarded_message_id" BIGINT NOT NULL UNIQUE,
	"image_filename" TEXT
);

CREATE TABLE "sent_messages" (
	"id" BIGINT PRIMARY KEY,
	"thread_id" BIGINT NOT NULL REFERENCES "threads" ON DELETE CASCADE,
	"forwarded_message_id" BIGINT NOT NULL UNIQUE,
	"author_id" BIGINT NOT NULL,
	"anonymous" BOOLEAN NOT NULL,
	"image_filename" TEXT
)