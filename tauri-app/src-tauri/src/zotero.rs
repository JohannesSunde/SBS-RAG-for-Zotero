// src-tauri/src/zotero.rs
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoteroItem {
    pub item_id: String,
    pub key: String,
    pub title: String,
    pub date: String,
    pub authors: String,
    pub tags: String,
    pub collections: String,
    pub attachment_key: String,
    pub attachment_path: String,
    pub item_type: String,
    pub pdf_path: Option<String>,
}

pub struct ZoteroLibrary {
    db_path: PathBuf,
}

impl ZoteroLibrary {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Self {
        Self {
            db_path: db_path.as_ref().to_path_buf(),
        }
    }

    pub fn get_connection(&self) -> Result<Connection> {
        // Open in read-only mode to avoid locking Zotero
        Connection::open_with_flags(
            &self.db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
        )
    }

    pub fn search_parent_items_with_pdfs(&self) -> Result<Vec<ZoteroItem>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT 
                i.itemID,
                i.key,
                v_title.value AS title,
                v_date.value AS date,
                GROUP_CONCAT(DISTINCT cr.lastName) AS authors,
                GROUP_CONCAT(DISTINCT t.name) AS tags,
                GROUP_CONCAT(DISTINCT c.collectionName) AS collections,
                att.key AS attachment_key,
                att_path.path AS attachment_path,
                itemType.typeName AS item_type
            FROM items i
            JOIN itemTypes itemType ON i.itemTypeID = itemType.itemTypeID
            JOIN itemCreators ic ON i.itemID = ic.itemID
            JOIN creators cr ON ic.creatorID = cr.creatorID
            JOIN itemData d_title ON i.itemID = d_title.itemID AND d_title.fieldID = (SELECT fieldID FROM fields WHERE fieldName = 'title')
            JOIN itemDataValues v_title ON d_title.valueID = v_title.valueID
            LEFT JOIN itemData d_date ON i.itemID = d_date.itemID AND d_date.fieldID = (SELECT fieldID FROM fields WHERE fieldName = 'date')
            LEFT JOIN itemDataValues v_date ON d_date.valueID = v_date.valueID
            LEFT JOIN itemTags it ON i.itemID = it.itemID
            LEFT JOIN tags t ON it.tagID = t.tagID
            LEFT JOIN collectionItems ci ON i.itemID = ci.itemID
            LEFT JOIN collections c ON ci.collectionID = c.collectionID

            LEFT JOIN itemAttachments ia ON i.itemID = ia.parentItemID
            LEFT JOIN items att ON ia.itemID = att.itemID
            LEFT JOIN itemData att_data ON att.itemID = att_data.itemID AND att_data.fieldID = (SELECT fieldID FROM fields WHERE fieldName = 'mimeType')
            LEFT JOIN itemDataValues att_mime ON att_data.valueID = att_mime.valueID
            LEFT JOIN itemAttachments att_path ON att.itemID = att_path.itemID

            WHERE (att_mime.value = 'application/pdf' OR att_path.path LIKE '%.pdf')
            GROUP BY i.itemID"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ZoteroItem {
                item_id: row.get::<_, i64>(0)?.to_string(),
                key: row.get(1)?,
                title: row.get(2)?,
                date: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                authors: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                tags: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                collections: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                attachment_key: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
                attachment_path: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
                item_type: row.get::<_, Option<String>>(9)?.unwrap_or_default(),
                pdf_path: None, // Resolved below
            })
        })?;

        let db_parent = self.db_path.parent().unwrap();
        let storage_dir = db_parent.join("storage");
        
        let mut items = Vec::new();
        for item_res in rows {
            let mut item = item_res?;
            
            // Resolve PDF Path logic
            if !item.attachment_key.is_empty() && !item.attachment_path.is_empty() {
                let raw_path = &item.attachment_path;
                let mut resolved_path = None;

                if raw_path.starts_with("storage:") {
                    let filename = raw_path.trim_start_matches("storage:");
                    resolved_path = Some(storage_dir.join(&item.attachment_key).join(filename));
                } else if Path::new(raw_path).is_absolute() {
                    resolved_path = Some(PathBuf::from(raw_path));
                } else if raw_path.starts_with("attachments:") {
                    // This often refers to a base directory set in Zotero, but for now 
                    // we'll try to find it. In a real app, we'd need the base dir setting.
                    let relative = raw_path.trim_start_matches("attachments:");
                    resolved_path = Some(storage_dir.join(relative));
                } else {
                    resolved_path = Some(storage_dir.join(&item.attachment_key).join(raw_path));
                }

                if let Some(path) = resolved_path {
                    item.pdf_path = path.to_str().map(|s| s.to_string());
                }
            }
            
            items.push(item);
        }

        Ok(items)
    }
}
