use super::received_messages::ReceivedMessage;

pub struct SentMessage {
	pub id: u64,
	pub thread_id: u64,
	pub forwarded_message_id: u64,
	pub anonymous: bool,
}

pub async fn get_sent_message(
	pg: &tokio_postgres::Client,
	id: u64,
) -> eyre::Result<Option<SentMessage>> {
	let rows = pg
		.query(
			r#"
				SELECT "id", "thread_id", "forwarded_message_id", "anonymous"
				FROM "sent_messages"
				WHERE "id" = $1
			"#,
			&[&(id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.is_empty() {
		Ok(None)
	} else {
		let row = &rows[0];

		let id: i64 = row.get("id");
		let thread_id: i64 = row.get("thread_id");
		let fowarded_message_id: i64 = row.get("forwarded_message_id");
		let anonymous: bool = row.get("anonymous");

		Ok(Some(SentMessage {
			id: id as u64,
			thread_id: thread_id as u64,
			forwarded_message_id: fowarded_message_id as u64,
			anonymous,
		}))
	}
}

pub async fn insert_sent_message(
	pg: &tokio_postgres::Client,
	message: SentMessage,
) -> eyre::Result<()> {
	pg.execute(
		r#"
			INSERT INTO "sent_messages" ("id", "thread_id", "forwarded_message_id", "anonymous")
			VALUES ($1, $2, $3, $4)
		"#,
		&[
			&(message.id as i64),
			&(message.thread_id as i64),
			&(message.forwarded_message_id as i64),
			&message.anonymous,
		],
	)
	.await?;

	Ok(())
}

pub async fn delete_sent_message(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<()> {
	pg.execute(
		r#"
			DELETE FROM "sent_messages"
			WHERE "id" = $1
		"#,
		&[&(id as i64)],
	)
	.await?;

	Ok(())
}
