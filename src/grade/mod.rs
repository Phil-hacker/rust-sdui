
#[derive(Debug,Deserialize)]
pub struct Grade {
    pub id: u64,
    pub school_id: u64,
    pub shortcut: String,
    pub name: String,
    pub description: Option<String>,
    pub level: Option<String>,
    pub future_shortcut: Option<String>,
    pub bookable_id: Option<u64>,
    pub migrate_at: Option<u64>
}