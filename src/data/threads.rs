#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

pub struct Thread {
	pub id: u64,
	pub dm_channel_id: u64,
	pub user_id: u64,
	pub opened_by_id: u64,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Thread {
	fn from_row(row: &tokio_postgres::Row) -> Self {
		let id: i64 = row.get("id");
		let dm_channel_id: i64 = row.get("dm_channel_id");
		let user_id: i64 = row.get("user_id");
		let opened_by_id: i64 = row.get("opened_by_id");
		let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

		Self {
			id: id as u64,
			dm_channel_id: dm_channel_id as u64,
			user_id: user_id as u64,
			opened_by_id: opened_by_id as u64,
			created_at,
		}
	}
}

pub async fn get(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<Option<Thread>> {
	let rows = pg
		.query(
			r#"
				SELECT * FROM "threads"
				WHERE "id" = $1
			"#,
			&[&(id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.len() == 1 {
		Ok(Some(Thread::from_row(&rows[0])))
	} else {
		Ok(None)
	}
}

pub async fn get_by_dm(
	pg: &tokio_postgres::Client,
	dm_channel_id: u64,
) -> eyre::Result<Option<Thread>> {
	let rows = pg
		.query(
			r#"
				SELECT * FROM "threads"
				WHERE "dm_channel_id" = $1
			"#,
			&[&(dm_channel_id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.len() == 1 {
		Ok(Some(Thread::from_row(&rows[0])))
	} else {
		Ok(None)
	}
}

pub async fn get_by_user(
	pg: &tokio_postgres::Client,
	user_id: u64,
) -> eyre::Result<Option<Thread>> {
	let rows = pg
		.query(
			r#"
				SELECT * FROM "threads"
				WHERE "user_id" = $1
			"#,
			&[&(user_id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.len() == 1 {
		Ok(Some(Thread::from_row(&rows[0])))
	} else {
		Ok(None)
	}
}

pub async fn insert(pg: &tokio_postgres::Client, thread: Thread) -> eyre::Result<()> {
	pg.query(
		r#"
			INSERT INTO "threads" ("id", "dm_channel_id", "user_id", "opened_by_id", "created_at")
			VALUES ($1, $2, $3, $4, $5)
		"#,
		&[
			&(thread.id as i64),
			&(thread.dm_channel_id as i64),
			&(thread.user_id as i64),
			&(thread.opened_by_id as i64),
			&thread.created_at,
		],
	)
	.await?;

	Ok(())
}

pub async fn delete(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<()> {
	pg.query(
		r#"
			DELETE FROM "threads"
			WHERE "id" = $1
		"#,
		&[&(id as i64)],
	)
	.await?;

	Ok(())
}
