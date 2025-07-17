#![allow(clippy::cast_possible_wrap)]

pub async fn add(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<bool> {
	let count = pg
		.execute(
			r#"
				INSERT INTO "blocked_users" ("id")
				VALUES ($1)
				ON CONFLICT DO NOTHING
			"#,
			&[&(id as i64)],
		)
		.await?;

	Ok(count != 0)
}

pub async fn remove(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<bool> {
	let count = pg
		.execute(
			r#"
				DELETE FROM "blocked_users"
				WHERE "id" = $1
			"#,
			&[&(id as i64)],
		)
		.await?;

	Ok(count != 0)
}

pub async fn has(pg: &tokio_postgres::Client, id: u64) -> eyre::Result<bool> {
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
