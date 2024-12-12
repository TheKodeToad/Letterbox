pub async fn get_thread_dm_channel(
	pg: &tokio_postgres::Client,
	id: u64,
) -> eyre::Result<Option<u64>> {
	let rows = pg
		.query(
			r#"
			SELECT "dm_channel_id"
			FROM "threads"
			WHERE "id" = $1
		"#,
			&[&(id as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.len() == 1 {
		Ok(Some(rows[0].get::<_, i64>("dm_channel_id") as u64))
	} else {
		Ok(None)
	}
}

pub async fn get_thread_id(
	pg: &tokio_postgres::Client,
	dm_channel: u64,
) -> eyre::Result<Option<u64>> {
	let rows = pg
		.query(
			r#"
				SELECT "id"
				FROM "threads"
				WHERE "dm_channel_id" = $1
			"#,
			&[&(dm_channel as i64)],
		)
		.await?;

	assert!(rows.len() <= 1);

	if rows.is_empty() {
		Ok(None)
	} else {
		Ok(Some(rows[0].get::<_, i64>("id") as u64))
	}
}

pub async fn insert_thread(
	pg: &tokio_postgres::Client,
	id: u64,
	dm_channel_id: u64,
) -> eyre::Result<()> {
	pg.query(
		r#"
			INSERT INTO "threads" ("id", "dm_channel_id")
			VALUES ($1, $2)
		"#,
		&[&(id as i64), &(dm_channel_id as i64)],
	)
	.await?;

	Ok(())
}

pub async fn delete_thread(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<()> {
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
