pub async fn get(pg: &tokio_postgres::Client, name: &str) -> eyre::Result<Option<String>> {
	let rows = pg
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

pub async fn search(pg: &tokio_postgres::Client, filter: &str) -> eyre::Result<Vec<String>> {
	let rows = pg
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

pub async fn set(pg: &tokio_postgres::Client, name: &String, content: &String) -> eyre::Result<()> {
	pg.execute(
		r#"
			INSERT INTO "tags" VALUES ($1, $2)
			ON CONFLICT ("name") DO UPDATE SET "name" = $1, "content" = $2
		"#,
		&[name, content],
	)
	.await?;

	Ok(())
}

pub async fn delete(pg: &tokio_postgres::Client, name: &String) -> eyre::Result<bool> {
	let count = pg
		.execute(
			// RETURNING needed otherwise we can't check with tokio-postgres (AFAIK)
			r#"
				DELETE FROM "tags"
				WHERE "name" = $1
			"#,
			&[name],
		)
		.await?;

	Ok(count != 0)
}
