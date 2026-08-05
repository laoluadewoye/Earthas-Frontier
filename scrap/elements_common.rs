impl <T: EFComponent> EFComponent for Box<T> {
    fn get_component_str(&self) -> String {
        format!("box_{}", self.into())
    }
}

impl <T: EFComponent> EFComponent for Vec<T> {
    fn get_component_str() -> String {
        format!("vector_{}", T::get_component_str().as_str())
    }
}

impl <K: EFComponent, V: EFComponent> EFComponent for HashMap<K, V> {
    fn get_component_str() -> String {
        format!(
            "hashmap_{}_to_{}", 
            K::get_component_str().as_str(), 
            V::get_component_str().as_str()
        )
    }
}
