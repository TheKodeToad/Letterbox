#![allow(clippy::cast_possible_wrap)]

pub async fn add(pool: &deadpool_postgres::Pool, id: u64) -> eyre::Result<bool> {
	let client = pool.get().await?;

	let count = client
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

pub async fn remove(pool: &deadpool_postgres::Pool, id: u64) -> eyre::Result<bool> {
	let client = pool.get().await?;

	let count = client
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

pub async fn has(pool: &deadpool_postgres::Pool, id: u64) -> eyre::Result<bool> {
	let client = pool.get().await?;

	let rows = client
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
