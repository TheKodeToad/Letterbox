pub async fn block_user(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<()> {
	pg.execute(
		r#"
			INSERT INTO "blocked_users" ("id")
			VALUES ($1)
			ON CONFLICT DO NOTHING
		"#,
		&[&(id as i64)],
	)
	.await?;

	Ok(())
}

pub async fn unblock_user(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<()> {
	pg.execute(
		r#"
			DELETE FROM "blocked_users"
			WHERE "id" = $1
		"#,
		&[&(id as i64)],
	)
	.await?;

	Ok(())
}

pub async fn is_user_blocked(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<bool> {
	let rows = pg
		.query(
			r#"
			SELECT 1 FROM "blocked_users"
			WHERE id = $1
		"#,
			&[&(id as i64)],
		)
		.await?;

	Ok(!rows.is_empty())
}
