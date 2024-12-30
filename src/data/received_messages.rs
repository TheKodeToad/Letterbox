#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

pub struct ReceivedMessage {
	pub id: u64,
	pub thread_id: u64,
	pub forwarded_message_id: u64,
	pub image_filename: Option<String>,
}

impl ReceivedMessage {
	fn from_row(row: &tokio_postgres::Row) -> Self {
		let id: i64 = row.get("id");
		let thread_id: i64 = row.get("thread_id");
		let fowarded_message_id: i64 = row.get("forwarded_message_id");
		let image_filename: Option<String> = row.get("image_filename");

		ReceivedMessage {
			id: id as u64,
			thread_id: thread_id as u64,
			forwarded_message_id: fowarded_message_id as u64,
			image_filename,
		}
	}
}

pub async fn get_received_message(
	pg: &tokio_postgres::Client,
	id: u64,
) -> eyre::Result<Option<ReceivedMessage>> {
	let rows = pg
		.query(
			r#"
				SELECT *
				FROM "received_messages"
				WHERE "id" = $1
			"#,
			&[&(id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.is_empty() {
		Ok(None)
	} else {
		Ok(Some(ReceivedMessage::from_row(&rows[0])))
	}
}

pub async fn insert_received_message(
	pg: &tokio_postgres::Client,
	message: ReceivedMessage,
) -> eyre::Result<()> {
	pg.execute(
		r#"
			INSERT INTO "received_messages" ("id", "thread_id", "forwarded_message_id", "image_filename")
			VALUES ($1, $2, $3, $4)
		"#,
		&[
			&(message.id as i64),
			&(message.thread_id as i64),
			&(message.forwarded_message_id as i64),
			&message.image_filename,
		],
	)
	.await?;

	Ok(())
}
