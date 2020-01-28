use std::collections::HashMap;

pub struct Task {
    pub name: String,
    pub progress: u32,
    pub steps: u32,
}

impl Task {
    pub fn print(&self) {
        println!("{}: {} of {} steps complete", self.name, self.progress, self.steps);
    }

    pub fn incr(&mut self) {
        if self.progress < self.steps {
            self.progress += 1;
        }
    }

    pub fn decr(&mut self) {
        if self.progress > 0 {
            self.progress -= 1;
        }
    }

    // TODO pub fn rename(&mut self, new_name: String) { ... }
}

pub struct Tasks {
    pub tasks: HashMap<String, Task>,
}

impl Tasks {
    pub fn add(&mut self, name: String, progress: u32, steps: u32) {
        let task_name = String::from(&name);
        let task = Task {
            name: name,
            progress: progress,
            steps: steps,
        };

        self.tasks.insert(task_name, task);
    }

    pub fn remove(&mut self, name: String) {
        // Wrapped with a function so it can be called directly by QML
        if let Some(_t) = self.tasks.remove(&name) {
            // removed successful
        }
    }

    pub fn incr(&mut self, name: String) {
        if let Some(t) = self.tasks.get_mut(&name) {
            t.incr();
        }
    }

    pub fn decr(&mut self, name: String) {
        if let Some(t) = self.tasks.get_mut(&name) {
            t.decr();
        }
    }

    pub fn print(&self) {
        println!("");
        for (_key, m_task) in &self.tasks {
            m_task.print();
        }
    }

    pub fn init(&mut self) {
        self.tasks = HashMap::new();
    }
}
