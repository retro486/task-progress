#[macro_use]
extern crate cstr;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use qmetaobject::*;
use std::collections::HashMap;

mod qrc;

#[derive(QObject,Default)]
struct Greeter {
    base : qt_base_class!(trait QObject),
    name : qt_property!(QString; NOTIFY name_changed),
    name_changed : qt_signal!(),
    compute_greetings : qt_method!(fn compute_greetings(&self, verb : String) -> QString {
        return (verb + " " + &self.name.to_string()).into()
    })
}

struct Task {
    name: String,
    progress: u32,
    steps: u32,
}

impl Task {
    fn print(&self) {
        println!("{}: {} of {} steps complete", self.name, self.progress, self.steps);
    }

    fn incr(&mut self) {
        if self.progress < self.steps {
            self.progress += 1;
        }
    }

    fn decr(&mut self) {
        if self.progress > 0 {
            self.progress -= 1;
        }
    }
}

struct Tasks {
    tasks: HashMap<String, Task>,
}

impl Tasks {
    fn add(&mut self, name: String, progress: u32, steps: u32) {
        let task_name = String::from(&name);
        let task = Task {
            name: name,
            progress: progress,
            steps: steps,
        };

        self.tasks.insert(task_name, task);
    }

    fn remove(&mut self, name: String) {
        // Wrapped with a function so it can be called directly by QML
        if let Some(_t) = self.tasks.remove(&name) {
            // removed successful
        }
    }

    fn incr(&mut self, name: String) {
        if let Some(t) = self.tasks.get_mut(&name) {
            t.incr();
        }
    }

    fn decr(&mut self, name: String) {
        if let Some(t) = self.tasks.get_mut(&name) {
            t.decr();
        }
    }

    fn print(&self) {
        println!("");
        for (_key, m_task) in &self.tasks {
            m_task.print();
        }
    }
}

fn main() {
    let mut tasks = Tasks {
        tasks: HashMap::new(),
    };
    
    unsafe {
        cpp! { {
            #include <QtCore/QCoreApplication>
            #include <QtCore/QString>
        }}
        cpp!{[]{
            QCoreApplication::setApplicationName(QStringLiteral("retro486-task-progress.retro486"));
        }}
    }
    QQuickStyle::set_style("Suru");
    qrc::load();
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/qml/Main.qml".into());
    engine.exec();
}
