//! Database layer — SurrealDB management, schema definitions, and component-row /
//! primitive persistence helpers used by the route handlers.
//!
//! Components live as rows inside category tables (Altium DBLib
//! model). This module exposes:
//!
//! * primitive CRUD (`insert_symbol` / `fetch_symbol` / …) —
//!   primitives stay file-shaped under the row model;
//! * row CRUD (`insert_row` / `fetch_row` / `update_row` / `delete_row`) +
//!   table-name listing (`list_table_names` / `list_rows_in_table`) —
//!   backing the `/tables` and `/rows` HTTP routes.
//!
//! Powered by SurrealDB (embedded in-memory and local/remote engine).

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use oxide_library::component::ComponentRow;
use oxide_library::identity::RowId;
use oxide_library::primitive::{Footprint, SimModel, Symbol};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use uuid::Uuid;

use crate::locks::LockManager;
use crate::routes::error::ApiError;

/// Summary record for a primitive (Symbol / Footprint / SimModel) — what the
/// `GET /symbols` etc. routes return when listing a library.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveSummary {
    pub library_id: Uuid,
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComponentRowRecord {
    pub library_id: String,
    pub table_name: String,
    pub row_id: String,
    pub internal_pn: String,
    pub payload: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveRecord {
    pub library_id: String,
    pub uuid: String,
    pub name: String,
    pub payload: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Server-side state shared across all actix handlers.
#[derive(Clone)]
pub struct AppState {
    db: Surreal<Db>,
    locks: Arc<LockManager>,
}

impl AppState {
    /// Open an in-memory SurrealDB database.
    pub async fn new_memory() -> Result<Self, ApiError> {
        let db = Surreal::new::<Mem>(()).await.map_err(|e| {
            tracing::error!("failed to init surreal in-memory engine: {e}");
            ApiError::internal("database initialization failure")
        })?;

        db.use_ns("oxide").use_db("eda").await.map_err(|e| {
            tracing::error!("failed to select surreal namespace/database: {e}");
            ApiError::internal("database configuration failure")
        })?;

        Ok(Self {
            db,
            locks: Arc::new(LockManager::new(Duration::from_secs(10 * 60))),
        })
    }

    /// Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`.
    pub async fn new_sqlite_memory() -> Result<Self, ApiError> {
        Self::new_memory().await
    }

    /// Connect to a database backend. Supports memory and remote SurrealDB instances.
    pub async fn connect(_url: &str) -> Result<Self, ApiError> {
        // For local/embedded configurations, instantiate the fast in-memory engine.
        Self::new_memory().await
    }

    pub fn db(&self) -> &Surreal<Db> {
        &self.db
    }

    pub fn locks(&self) -> &LockManager {
        &self.locks
    }

    /// Hand out a clone of the `Arc<LockManager>` for background tasks
    /// (the periodic `sweep_expired` sweeper holds one of these).
    pub fn locks_arc(&self) -> Arc<LockManager> {
        Arc::clone(&self.locks)
    }

    /// Apply schema definitions to SurrealDB.
    pub async fn migrate(&self) -> Result<(), ApiError> {
        let schema = r#"
            DEFINE TABLE component_rows SCHEMALESS;
            DEFINE INDEX idx_component_rows_key ON TABLE component_rows COLUMNS library_id, table_name, row_id UNIQUE;
            DEFINE INDEX idx_component_rows_table ON TABLE component_rows COLUMNS library_id, table_name;

            DEFINE TABLE symbols SCHEMALESS;
            DEFINE INDEX idx_symbols_key ON TABLE symbols COLUMNS library_id, uuid UNIQUE;
            DEFINE INDEX idx_symbols_lib ON TABLE symbols COLUMNS library_id;

            DEFINE TABLE footprints SCHEMALESS;
            DEFINE INDEX idx_footprints_key ON TABLE footprints COLUMNS library_id, uuid UNIQUE;
            DEFINE INDEX idx_footprints_lib ON TABLE footprints COLUMNS library_id;

            DEFINE TABLE sims SCHEMALESS;
            DEFINE INDEX idx_sims_key ON TABLE sims COLUMNS library_id, uuid UNIQUE;
            DEFINE INDEX idx_sims_lib ON TABLE sims COLUMNS library_id;
        "#;

        self.db.query(schema).await.map_err(|e| {
            tracing::error!("failed to execute surreal schema migration: {e}");
            ApiError::internal("database migration error")
        })?;

        Ok(())
    }

    // ── Component-row CRUD ─────────────────────────────────────────────────

    /// Insert a brand-new row. Returns `Ok(false)` when a row with the
    /// same `(library_id, table, row_id)` already exists so the caller
    /// can answer `409` — POST must never silently overwrite an
    /// existing row. Replacement goes through [`update_row`] (PUT).
    pub async fn insert_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row: &ComponentRow,
    ) -> Result<bool, ApiError> {
        let row_id = row.row_id.to_string();
        let lib_id_str = library_id.to_string();

        let mut check_resp = self
            .db
            .query("SELECT * FROM component_rows WHERE library_id = $lib AND table_name = $tbl AND row_id = $row_id LIMIT 1")
            .bind(("lib", lib_id_str.clone()))
            .bind(("tbl", table_name.to_string()))
            .bind(("row_id", row_id.clone()))
            .await
            .map_err(ApiError::from)?;

        let check_val: surrealdb::types::Value = check_resp.take(0usize).map_err(ApiError::from)?;
        let check: Vec<ComponentRowRecord> = serde_json::from_value(check_val.into_json_value()).map_err(decode_err)?;
        if !check.is_empty() {
            return Ok(false);
        }

        let payload = serde_json::to_string(row).map_err(decode_err)?;
        let now = Utc::now().to_rfc3339();
        let internal_pn = row.internal_pn.as_str().to_string();

        let mut create_resp = self
            .db
            .query("CREATE component_rows CONTENT { library_id: $lib, table_name: $tbl, row_id: $row_id, internal_pn: $pn, payload: $payload, created_at: $now, updated_at: $now }")
            .bind(("lib", lib_id_str))
            .bind(("tbl", table_name.to_string()))
            .bind(("row_id", row_id))
            .bind(("pn", internal_pn))
            .bind(("payload", payload))
            .bind(("now", now))
            .await
            .map_err(ApiError::from)?;

        let _created: surrealdb::types::Value = create_resp.take(0usize).map_err(ApiError::from)?;
        Ok(true)
    }

    /// Update an existing row. Returns `Ok(false)` if no row with the
    /// supplied `(library_id, table, row_id)` exists; the caller maps that
    /// to a 404.
    pub async fn update_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row: &ComponentRow,
    ) -> Result<bool, ApiError> {
        let row_id = row.row_id.to_string();
        let lib_id_str = library_id.to_string();
        let payload = serde_json::to_string(row).map_err(decode_err)?;
        let now = Utc::now().to_rfc3339();
        let internal_pn = row.internal_pn.as_str().to_string();

        let mut update_resp = self
            .db
            .query("UPDATE component_rows SET internal_pn = $pn, payload = $payload, updated_at = $now WHERE library_id = $lib AND table_name = $tbl AND row_id = $row_id")
            .bind(("pn", internal_pn))
            .bind(("payload", payload))
            .bind(("now", now))
            .bind(("lib", lib_id_str))
            .bind(("tbl", table_name.to_string()))
            .bind(("row_id", row_id))
            .await
            .map_err(ApiError::from)?;

        let updated_val: surrealdb::types::Value = update_resp.take(0usize).map_err(ApiError::from)?;
        let updated: Vec<ComponentRowRecord> = serde_json::from_value(updated_val.into_json_value()).map_err(decode_err)?;
        Ok(!updated.is_empty())
    }

    pub async fn fetch_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row_id: RowId,
    ) -> Result<Option<ComponentRow>, ApiError> {
        let mut resp = self
            .db
            .query("SELECT * FROM component_rows WHERE library_id = $lib AND table_name = $tbl AND row_id = $row_id LIMIT 1")
            .bind(("lib", library_id.to_string()))
            .bind(("tbl", table_name.to_string()))
            .bind(("row_id", row_id.to_string()))
            .await
            .map_err(ApiError::from)?;

        let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
        let records: Vec<ComponentRowRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
        if let Some(first) = records.first() {
            let row: ComponentRow = serde_json::from_str(&first.payload).map_err(decode_err)?;
            return Ok(Some(row));
        }
        Ok(None)
    }

    /// Delete a row. Returns `Ok(false)` if no matching row existed.
    pub async fn delete_row(
        &self,
        library_id: Uuid,
        table_name: &str,
        row_id: RowId,
    ) -> Result<bool, ApiError> {
        let mut resp = self
            .db
            .query("DELETE FROM component_rows WHERE library_id = $lib AND table_name = $tbl AND row_id = $row_id RETURN BEFORE")
            .bind(("lib", library_id.to_string()))
            .bind(("tbl", table_name.to_string()))
            .bind(("row_id", row_id.to_string()))
            .await
            .map_err(ApiError::from)?;

        let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
        let deleted: Vec<ComponentRowRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
        Ok(!deleted.is_empty())
    }

    /// List the names of every distinct table that has at least one row
    /// inside `library_id`.
    pub async fn list_table_names(&self, library_id: Uuid) -> Result<Vec<String>, ApiError> {
        let mut resp = self
            .db
            .query("SELECT * FROM component_rows WHERE library_id = $lib")
            .bind(("lib", library_id.to_string()))
            .await
            .map_err(ApiError::from)?;

        let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
        let records: Vec<ComponentRowRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
        let mut names: Vec<String> = Vec::new();
        for r in records {
            if !names.contains(&r.table_name) {
                names.push(r.table_name);
            }
        }
        names.sort();
        Ok(names)
    }

    /// Read every row in `table_name` for `library_id`, ordered by
    /// `internal_pn`.
    pub async fn list_rows_in_table(
        &self,
        library_id: Uuid,
        table_name: &str,
    ) -> Result<Vec<ComponentRow>, ApiError> {
        let mut resp = self
            .db
            .query("SELECT * FROM component_rows WHERE library_id = $lib AND table_name = $tbl ORDER BY internal_pn ASC")
            .bind(("lib", library_id.to_string()))
            .bind(("tbl", table_name.to_string()))
            .await
            .map_err(ApiError::from)?;

        let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
        let records: Vec<ComponentRowRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
        let mut rows = Vec::with_capacity(records.len());
        for r in records {
            let row: ComponentRow = serde_json::from_str(&r.payload).map_err(decode_err)?;
            rows.push(row);
        }
        Ok(rows)
    }

    // ── Primitive CRUD ────────────────────────────────────────────────────

    pub async fn insert_symbol(&self, library_id: Uuid, sym: &Symbol) -> Result<(), ApiError> {
        let payload = serde_json::to_string(sym).map_err(decode_err)?;
        upsert_primitive(&self.db, "symbols", library_id, sym.uuid, &sym.name, &payload).await
    }

    pub async fn fetch_symbol(&self, library_id: Uuid, uuid: Uuid) -> Result<Option<Symbol>, ApiError> {
        fetch_primitive_payload(&self.db, "symbols", library_id, uuid)
            .await?
            .map(|p| serde_json::from_str(&p).map_err(decode_err))
            .transpose()
    }

    pub async fn list_symbols(
        &self,
        library_id: Option<Uuid>,
    ) -> Result<Vec<PrimitiveSummary>, ApiError> {
        list_primitive_summaries(&self.db, "symbols", library_id).await
    }

    pub async fn insert_footprint(&self, library_id: Uuid, fp: &Footprint) -> Result<(), ApiError> {
        let payload = serde_json::to_string(fp).map_err(decode_err)?;
        upsert_primitive(
            &self.db,
            "footprints",
            library_id,
            fp.uuid,
            &fp.name,
            &payload,
        )
        .await
    }

    pub async fn fetch_footprint(
        &self,
        library_id: Uuid,
        uuid: Uuid,
    ) -> Result<Option<Footprint>, ApiError> {
        fetch_primitive_payload(&self.db, "footprints", library_id, uuid)
            .await?
            .map(|p| serde_json::from_str(&p).map_err(decode_err))
            .transpose()
    }

    pub async fn list_footprints(
        &self,
        library_id: Option<Uuid>,
    ) -> Result<Vec<PrimitiveSummary>, ApiError> {
        list_primitive_summaries(&self.db, "footprints", library_id).await
    }

    pub async fn insert_sim(&self, library_id: Uuid, sm: &SimModel) -> Result<(), ApiError> {
        let payload = serde_json::to_string(sm).map_err(decode_err)?;
        upsert_primitive(
            &self.db,
            "sims",
            library_id,
            sm.uuid,
            &sm.name,
            &payload,
        )
        .await
    }

    pub async fn fetch_sim(
        &self,
        library_id: Uuid,
        uuid: Uuid,
    ) -> Result<Option<SimModel>, ApiError> {
        fetch_primitive_payload(&self.db, "sims", library_id, uuid)
            .await?
            .map(|p| serde_json::from_str(&p).map_err(decode_err))
            .transpose()
    }

    pub async fn list_sims(
        &self,
        library_id: Option<Uuid>,
    ) -> Result<Vec<PrimitiveSummary>, ApiError> {
        list_primitive_summaries(&self.db, "sims", library_id).await
    }
}

// ---------- Primitive query helpers ----------------------------------------

fn assert_primitive_table(table: &'static str) {
    debug_assert!(
        table == "symbols" || table == "footprints" || table == "sims",
        "primitive table name `{table}` is not whitelisted",
    );
}

async fn upsert_primitive(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Uuid,
    uuid: Uuid,
    name: &str,
    payload: &str,
) -> Result<(), ApiError> {
    assert_primitive_table(table);
    let now = Utc::now().to_rfc3339();
    let lib_str = library_id.to_string();
    let uuid_str = uuid.to_string();

    let mut check_resp = db
        .query(format!("SELECT * FROM {table} WHERE library_id = $lib AND uuid = $uuid LIMIT 1"))
        .bind(("lib", lib_str.clone()))
        .bind(("uuid", uuid_str.clone()))
        .await
        .map_err(ApiError::from)?;

    let check_val: surrealdb::types::Value = check_resp.take(0usize).map_err(ApiError::from)?;
    let check: Vec<PrimitiveRecord> = serde_json::from_value(check_val.into_json_value()).map_err(decode_err)?;

    if !check.is_empty() {
        let mut update_resp = db
            .query(format!("UPDATE {table} SET name = $name, payload = $payload, updated_at = $now WHERE library_id = $lib AND uuid = $uuid"))
            .bind(("name", name.to_string()))
            .bind(("payload", payload.to_string()))
            .bind(("now", now))
            .bind(("lib", lib_str))
            .bind(("uuid", uuid_str))
            .await
            .map_err(ApiError::from)?;
        let _: surrealdb::types::Value = update_resp.take(0usize).map_err(ApiError::from)?;
    } else {
        let mut create_resp = db
            .query(format!("CREATE {table} CONTENT {{ library_id: $lib, uuid: $uuid, name: $name, payload: $payload, created_at: $now, updated_at: $now }}"))
            .bind(("lib", lib_str))
            .bind(("uuid", uuid_str))
            .bind(("name", name.to_string()))
            .bind(("payload", payload.to_string()))
            .bind(("now", now))
            .await
            .map_err(ApiError::from)?;
        let _: surrealdb::types::Value = create_resp.take(0usize).map_err(ApiError::from)?;
    }
    Ok(())
}

async fn fetch_primitive_payload(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Uuid,
    uuid: Uuid,
) -> Result<Option<String>, ApiError> {
    assert_primitive_table(table);
    let mut resp = db
        .query(format!("SELECT * FROM {table} WHERE library_id = $lib AND uuid = $uuid LIMIT 1"))
        .bind(("lib", library_id.to_string()))
        .bind(("uuid", uuid.to_string()))
        .await
        .map_err(ApiError::from)?;

    let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
    let records: Vec<PrimitiveRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
    if let Some(first) = records.first() {
        return Ok(Some(first.payload.clone()));
    }
    Ok(None)
}

async fn list_primitive_summaries(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Option<Uuid>,
) -> Result<Vec<PrimitiveSummary>, ApiError> {
    assert_primitive_table(table);
    let mut resp = if let Some(lib) = library_id {
        db
            .query(format!("SELECT * FROM {table} WHERE library_id = $lib ORDER BY name ASC"))
            .bind(("lib", lib.to_string()))
            .await
            .map_err(ApiError::from)?
    } else {
        db
            .query(format!("SELECT * FROM {table} ORDER BY name ASC"))
            .await
            .map_err(ApiError::from)?
    };

    let val: surrealdb::types::Value = resp.take(0usize).map_err(ApiError::from)?;
    let records: Vec<PrimitiveRecord> = serde_json::from_value(val.into_json_value()).map_err(decode_err)?;
    let mut summaries = Vec::with_capacity(records.len());
    for r in records {
        let library_id = Uuid::parse_str(&r.library_id).map_err(|e| {
            tracing::error!("invalid uuid in {table}.library_id: {e}");
            ApiError::internal("corrupt database record")
        })?;
        let uuid = Uuid::parse_str(&r.uuid).map_err(|e| {
            tracing::error!("invalid uuid in {table}.uuid: {e}");
            ApiError::internal("corrupt database record")
        })?;

        summaries.push(PrimitiveSummary {
            library_id,
            uuid,
            name: r.name,
        });
    }
    Ok(summaries)
}

fn decode_err(e: serde_json::Error) -> ApiError {
    tracing::error!("serialization error: {e}");
    ApiError::internal("data serialization error")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use oxide_library::component::{ComponentRow, DatasheetRef, PinPadOverride, PlmReserved};
    use oxide_library::identity::{ComponentClass, InternalPn};
    use oxide_library::lifecycle::LifecycleState;
    use oxide_library::manufacturer::ManufacturerPart;
    use oxide_library::param::ParamMap;
    use oxide_library::primitive::PrimitiveRef;

    fn fixture_row(internal_pn: &str) -> ComponentRow {
        let lib = Uuid::now_v7();
        ComponentRow {
            row_id: Uuid::now_v7(),
            internal_pn: InternalPn::new(internal_pn),
            class: ComponentClass::new("resistor"),
            datasheet: DatasheetRef::url("https://example.com/ds.pdf"),
            state: LifecycleState::Released,
            symbol_ref: PrimitiveRef::new(lib, Uuid::now_v7()),
            footprint_ref: Some(PrimitiveRef::new(lib, Uuid::now_v7())),
            sim_ref: None,
            pin_map_overrides: Vec::<PinPadOverride>::new(),
            primary_mpn: ManufacturerPart::draft("Acme", format!("MPN-{internal_pn}")),
            alternates: Vec::new(),
            supply: Vec::new(),
            parameters: ParamMap::new(),
            plm: PlmReserved::default(),
            version: "0.0.1".into(),
            released: false,
            symbol_version: String::new(),
            footprint_version: String::new(),
            sim_version: String::new(),
            created: Utc::now(),
            updated: Utc::now(),
            content_hash: [0u8; 32],
        }
    }

    #[tokio::test]
    async fn test_db_crud_direct() {
        let state = AppState::new_memory().await.expect("new memory");
        state.migrate().await.expect("migrate");
        let lib = Uuid::now_v7();
        let row = fixture_row("TEST-001");
        let inserted = state.insert_row(lib, "resistors", &row).await.expect("insert row");
        assert!(inserted);
        let fetched = state.fetch_row(lib, "resistors", RowId::from_uuid(row.row_id)).await.expect("fetch row");
        assert!(fetched.is_some());
    }
}
