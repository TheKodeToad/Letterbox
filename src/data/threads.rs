pub async fn thread_target_from_source(
	pg: &tokio_postgres::Client,
	source: u64,
) -> eyre::Result<Option<u64>> {
	let rows = pg
		.query(
			r#"
			SELECT ("target_channel_id")
			FROM "threads"
			WHERE "source_channel_id" = $1
			LIMIT 1
		"#,
			&[&(source as i64)],
		)
		.await?;

	if rows.len() == 1 {
		Ok(Some(rows[0].get::<_, i64>("target_channel_id") as u64))
	} else {
		Ok(None)
	}
}

pub async fn thread_source_from_target(
	pg: &tokio_postgres::Client,
	target: u64,
) -> eyre::Result<Option<u64>> {
	let rows = pg
		.query(
			r#"
			SELECT ("source_channel_id")
			FROM "threads"
			WHERE "target_channel_id" = $1
		"#,
			&[&(target as i64)],
		)
		.await?;

	if rows.len() == 1 {
		Ok(Some(rows[0].get::<_, i64>("source_channel_id") as u64))
	} else {
		Ok(None)
	}
}

pub async fn link_thread_source_to_target(
	pg: &tokio_postgres::Client,
	source: u64,
	target: u64,
) -> eyre::Result<()> {
	pg.query(
		r#"
			INSERT INTO "threads" ("source_channel_id", "target_channel_id")
			VALUES ($1, $2)
		"#,
		&[&(source as i64), &(target as i64)],
	)
	.await?;

	Ok(())
}

pub async fn delete_thread_by_source(pg: &tokio_postgres::Client, source: u64) -> eyre::Result<()> {
	pg.query(
		r#"
			DELETE FROM "threads"
			WHERE "source_channel_id" = $1
		"#,
		&[&(source as i64)],
	)
	.await?;

	Ok(())
}
