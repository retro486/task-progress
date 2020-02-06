#[macro_use]
extern crate cstr;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use qmetaobject::*;

mod qrc;

#[derive(QGadget,Clone,Default,Debug)]
struct Task {
    name: String,
    progress: u32,
    steps: u32,
}

impl Task {
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

#[allow(non_snake_case)]
#[derive(QObject,Default)]
struct Tasks {
    base: qt_base_class!(trait QAbstractListModel),
    data: Vec<Task>,
    i_count: qt_property!(usize; NOTIFY countChanged),
    countChanged: qt_signal!(),
    
    incr: qt_method!(fn incr(&mut self, index: usize) {
        if let Some(t) = self.data.get_mut(index) {
            t.incr();
        }
    }),

    decr: qt_method!(fn decr(&mut self, index: usize) {
        if let Some(t) = self.data.get_mut(index) {
            t.decr();
        }
    }),

    count: qt_method!(fn count(&self) -> usize {
        return self.i_count;
    }),

    remove: qt_method!(fn remove(&mut self, index: usize) {
        // Wrapped with a function so it can be called directly by QML
        if index < self.data.len() {
            self.data.remove(index);
        }
    }),

    add_dummy: qt_method!(fn add_dummy(&mut self) {
        let name = format!("Test {}", self.i_count);
        let count = self.i_count;
        (self as &mut dyn QAbstractListModel).begin_insert_rows(count as i32, count as i32);
        self.add(String::from(&name), 0, 10);
        (self as &mut dyn QAbstractListModel).end_insert_rows();
    }),
}

// But we still need to implement the QAbstractListModel manually
impl QAbstractListModel for Tasks {
    fn row_count(&self) -> i32 {
        println!("{:?}", self.data.len());
        return self.data.len() as i32;
    }
    
    fn data(&self, index: QModelIndex, role:i32) -> QVariant {
        if role != USER_ROLE { return QVariant::default(); }
        // We use the QGadget::to_qvariant function
        let m_task = self.data.get(index.row() as usize);
        let item = self.data.get(index.row() as usize).map(|x|x.to_qvariant()).unwrap_or_default();
        //println!("Here's item: {:#?}", m_task);
        return item;
    }
    
    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        return vec![(USER_ROLE, QByteArray::from("name"))].into_iter().collect();
    }
}

impl Tasks {
    fn add(&mut self, name: String, progress: u32, steps: u32) {
        let task = Task {
            name: String::from(&name),
            progress: progress,
            steps: steps,
        };

        self.data.push(task);
        self.i_count = self.data.len();
    }
}

fn main() {
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
    let mut engine = QmlEngine::new();
    qml_register_type::<Tasks>(cstr!("Tasks"), 1, 0, cstr!("Tasks"));
    engine.load_file("qrc:/qml/Main.qml".into());
    engine.exec();
}