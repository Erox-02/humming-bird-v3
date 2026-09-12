use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sha2::{Sha256, Digest};
use crate::Session;
use crate::core::Pipeline;
use crate::schemas::Prcsres;

pub struct Sesman {
    sessions: RwLock<HashMap<String, Arc<RwLock<Session>>>>,
}

impl Sesman {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn Crtses(&self, intent: Option<&str>) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_string();
        let mut hasher = Sha256::new();
        hasher.update(timestamp.as_bytes());
        let id = format!("{:x}", hasher.finalize());
        let mut session = Session::new(id.clone());
        if let Some(intent) = intent {
            session = session.wth_int(intent);
        }

        self.sessions.write().unwrap()
            .insert(id.clone(), Arc::new(RwLock::new(session)));
        id
    }

    pub fn get_ses(&self, id: &str) -> Option<Arc<RwLock<Session>>> {
        self.sessions.read().unwrap()
            .get(id)
            .cloned()
    }

    pub fn rm_ses(&self, id: &str) -> bool {
        self.sessions.write().unwrap()
            .remove(id)
            .is_some()
    }

    pub fn ls_ses(&self) -> Vec<String> {
        self.sessions.read().unwrap()
            .keys()
            .cloned()
            .collect()
    }

    pub fn ses_cnt(&self) -> usize {
        self.sessions.read().unwrap().len()
    }

    pub fn prc_wses(
        &self,
        pipeline: &mut Pipeline,
        session_id: &str,
        text: &str,
    ) -> Option<Prcsres> {
        let session_arc = self.get_ses(session_id)?;
        
        let intent;
        {
            let session = session_arc.read().unwrap();
            intent = session.intent.clone();
        }
        
        let mut session = session_arc.write().unwrap();
        
        let result = pipeline.prc_wses(
            text,
            &mut session,
            intent.as_deref(),
        );
        
        Some(result)
    }

    pub fn res_wses(
        &self,
        pipeline: &mut Pipeline,
        session_id: &str,
        text: &str,
    ) -> Option<String> {
        let session_arc = self.get_ses(session_id)?;
        let session = session_arc.read().unwrap();
        
        Some(pipeline.res_wmd(text, session.gt_md()))
    }
}

impl Default for Sesman {
    fn default() -> Self {
        Self::new()
    }
}