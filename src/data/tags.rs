pub async fn get(pool: &deadpool_postgres::Pool, name: &str) -> eyre::Result<Option<String>> {
	let client = pool.get().await?;

	let rows = client
		.query(
			r#"
				SELECT "content" FROM "tags"
				WHERE "name" = $1
			"#,
			&[&name],
		)
		.await?;

	if rows.len() == 1 {
		let content: String = rows[0].get("content");

		Ok(Some(content))
	} else {
		Ok(None)
	}
}

pub async fn search(pool: &deadpool_postgres::Pool, filter: &str) -> eyre::Result<Vec<String>> {
	let client = pool.get().await?;

	let rows = client
		.query(
			r#"
				SELECT "name" FROM "tags"
				WHERE position($1 in "name") > 0
				ORDER BY "name"
				LIMIT 25
			"#,
			&[&filter],
		)
		.await?;

	Ok(rows.iter().map(|row| row.get("name")).collect())
}

pub async fn set(
	pool: &deadpool_postgres::Pool,
 name: &String, content: &String) -> eyre::Result<()> {
	let client = pool.get().await?;

	client.execute(
		r#"
			INSERT INTO "tags" VALUES ($1, $2)
			ON CONFLICT ("name") DO UPDATE SET "name" = $1, "content" = $2
		"#,
		&[name, content],
	)
	.await?;

	Ok(())
}

pub async fn delete(
	pool: &deadpool_postgres::Pool,
 name: &String) -> eyre::Result<bool> {
	let client = pool.get().await?;

	let count = client
		.execute(
			r#"
				DELETE FROM "tags"
				WHERE "name" = $1
			"#,
			&[name],
		)
		.await?;

	Ok(count != 0)
}
