use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub struct ZoteroItem {
    pub item_id: i32,
    pub title: String,
    pub year: String,
    pub pdf_path: Option<String>,
}

pub struct ZoteroLibrary {
    conn: Connection,
}

impl ZoteroLibrary {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(ZoteroLibrary { conn })
    }

    /// Fetches a simplified list of items with their attached PDF paths.
    /// This is a basic implementation to kickstart the Rust rewrite.
    /// In a full implementation, this would handle the complex EAV schema of Zotero.
    pub fn get_items_with_pdfs(&self) -> Result<Vec<ZoteroItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                items.itemID,
                COALESCE((SELECT value FROM itemData JOIN itemDataValues USING(valueID) WHERE itemData.itemID = items.itemID AND fieldID = 110), 'Untitled') as title,
                COALESCE((SELECT value FROM itemData JOIN itemDataValues USING(valueID) WHERE itemData.itemID = items.itemID AND fieldID = 14), '') as date
             FROM items 
             WHERE itemTypeID != 1" // Not an attachment
        )?;

        let item_iter = stmt.query_map([], |row| {
            Ok(ZoteroItem {
                item_id: row.get(0)?,
                title: row.get(1)?,
                year: row.get(2)?,
                pdf_path: None, // We would resolve this via Zotero's attachment table
            })
        })?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }
        
        Ok(items)
    }
}
