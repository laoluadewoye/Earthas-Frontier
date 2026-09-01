// NOTE: NEED TO ADD FUNCTIONALITY FOR INTERACTING WITH FILES AND RULES
#[derive(Debug)]
pub struct EFStaticEntity<T: EFComponent> {
    id: String,
    name: String,
    owner: EFIDEntityOrName,
    system: EFIDEntityOrName,
    date_created: EFUTCTimestamp,
    date_accessed: EFUTCTimestamp,
    date_modified: EFUTCTimestamp,
    rules: HashMap<EFIDEntityOrName, Vec<EFEntityRule>>,
    files: Vec<EFFile>,
    component: T,
    component_type: String,
}

impl <T: EFComponent> EFStaticEntity<T> {
    pub fn new(name: String, system: EFIDEntityOrName, component: T, salt: String, entity_hash: &String) -> EFStaticEntity<T> {
        let timestamp: EFUTCTimestamp = EFUTCTimestamp::get_timestamp_for_now();
        let hash: String = match get_hash(vec![system.to_type_and_string().1, &timestamp.to_string(), &salt], entity_hash) {
            Ok(h) => h.value,
            Err(e) => { panic!("{}", &e.to_string().as_str()); }
        };
        let component_type: String = component.get_component_str();

        EFStaticEntity { 
            id: hash,
            name,
            system, 
            date_created: timestamp.clone(),
            date_accessed: timestamp.clone(),
            date_modified: timestamp,
            rules: HashMap::new(),
            files: Vec::new(),
            component,
            component_type
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_system(&self) -> &EFIDEntityOrName {
        &self.system
    }

    pub fn get_date_created(&self) -> &EFUTCTimestamp {
        &self.date_created
    }

    pub fn get_date_accessed(&self) -> &EFUTCTimestamp {
        &self.date_accessed
    }

    pub fn set_date_accessed(&mut self, date_accessed: EFUTCTimestamp) {
        self.date_accessed = date_accessed;
    }

    pub fn get_date_modified(&self) -> &EFUTCTimestamp {
        &self.date_modified
    }

    pub fn set_date_modified(&mut self, date_modified: EFUTCTimestamp) {
        self.date_modified = date_modified;
    }

    pub fn get_component(&self) -> &T {
        &self.component
    }

    pub fn get_mutable_component(&mut self) -> &mut T {
        &mut self.component
    }

    pub fn get_component_type(&self) -> &String {
        &self.component_type
    }
}

pub struct EFStaticEntityTracker<T: EFComponent> {
    entities: HashMap<String, EFStaticEntity<T>>
}

impl<T: EFComponent> EFEntityTracker for EFStaticEntityTracker<T> {
    type EntityType = EFStaticEntity<T>;

    fn new() -> Self {
        EFStaticEntityTracker { entities: HashMap::new() }
    }

    fn add_entity(&mut self, entity: Self::EntityType) -> Option<Self::EntityType> {
        self.entities.insert(entity.get_id().clone(), entity)
    }

    fn pop_entity(&mut self, entity_id: &str) -> Result<EFOk<Self::EntityType>, EFError> {
        match self.entities.remove(entity_id) {
            Some(e) => Ok(EFOk { 
                value: e, 
                msg: format!("Popped entity {} from tracker.", entity_id)
            }),
            None => Err(EFError { 
                function: String::from("pop_entity"), 
                line: String::from("self.entities.remove(entity_id)"), 
                msg: format!("Could not find entity {} in tracker to pop.", entity_id)
            })
        }
    }

    fn get_entity(&self, entity_id: &str) -> Result<EFOk<&Self::EntityType>, EFError> {
        match self.entities.get(entity_id) {
            Some(e) => Ok(EFOk { 
                value: e, 
                msg: format!("Got entity {} from tracker.", entity_id)
            }),
            None => Err(EFError { 
                function: String::from("get_entity"),
                line: String::from("self.entities.get(entity_id)"), 
                msg: format!("Could not find entity {} in tracker to get.", entity_id)
            })
        }
    }

    fn get_mut_entity(&mut self, entity_id: &str) -> Result<EFOk<&mut Self::EntityType>, EFError> {
        match self.entities.get_mut(entity_id) {
            Some(e) => Ok(EFOk { 
                value: e, 
                msg: format!("Got mutable entity {} from tracker.", entity_id)
            }),
            None => Err(EFError { 
                function: String::from("get_mut_entity"),
                line: String::from("self.entities.get_mut(entity_id)"), 
                msg: format!("Could not find entity {} in tracker to get as mutable.", entity_id)
            })
        }
    }
}
