pub struct SentMessage {
	pub id: u64,
	pub thread_id: u64,
	pub forwarded_message_id: u64,
	pub author_id: u64,
	pub anonymous: bool,
	pub image_filename: Option<String>,
}

impl SentMessage {
	fn from_row(row: &tokio_postgres::Row) -> Self {
		let id: i64 = row.get("id");
		let thread_id: i64 = row.get("thread_id");
		let fowarded_message_id: i64 = row.get("forwarded_message_id");
		let author_id: i64 = row.get("author_id");
		let anonymous: bool = row.get("anonymous");
		let image_filename: Option<String> = row.get("image_filename");

		SentMessage {
			id: id as u64,
			thread_id: thread_id as u64,
			author_id: author_id as u64,
			forwarded_message_id: fowarded_message_id as u64,
			anonymous,
			image_filename,
		}
	}
}

pub async fn get_sent_message(
	pg: &tokio_postgres::Client,
	id: u64,
) -> eyre::Result<Option<SentMessage>> {
	let rows = pg
		.query(
			r#"
				SELECT * FROM "sent_messages"
				WHERE "id" = $1
			"#,
			&[&(id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.is_empty() {
		Ok(None)
	} else {
		Ok(Some(SentMessage::from_row(&rows[0])))
	}
}

pub async fn get_sent_message_by_forwarded_message(
	pg: &tokio_postgres::Client,
	forwarded_message_id: u64,
) -> eyre::Result<Option<SentMessage>> {
	let rows = pg
		.query(
			r#"
				SELECT *
				FROM "sent_messages"
				WHERE "forwarded_message_id" = $1
			"#,
			&[&(forwarded_message_id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.is_empty() {
		Ok(None)
	} else {
		Ok(Some(SentMessage::from_row(&rows[0])))
	}
}

pub async fn insert_sent_message(
	pg: &tokio_postgres::Client,
	message: SentMessage,
) -> eyre::Result<()> {
	pg.execute(
		r#"
			INSERT INTO "sent_messages" ("id", "thread_id", "forwarded_message_id", "author_id", "anonymous", "image_filename")
			VALUES ($1, $2, $3, $4, $5, $6)
		"#,
		&[
			&(message.id as i64),
			&(message.thread_id as i64),
			&(message.forwarded_message_id as i64),
			&(message.author_id as i64),
			&message.anonymous,
			&message.image_filename,
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
