pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> () {
        if let Some(_) = self.plugins.iter().position(|x| x.name() == plugin.name()) {
            panic!("Plugin with name '{}' already exists", plugin.name())
        }

        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(index) = self.plugins.iter().position(|x| x.name() == name) {
            let removed = self.plugins.remove(index);

            Some(removed)
        } else {
            None
        }
    }

    pub fn execute_all(&self) -> () {
        self.plugins.iter().for_each(|p| {
            p.execute();
        });
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
