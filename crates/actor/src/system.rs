
use tokio::sync::RwLock;

pub trait Event: Clone + Send + Sync + 'static {}use std::{any::Any, collections::HashMap, sync::Arc};

pub struct System {
    pub name: String,
    actors: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync + 'static>>>>,
}


