import QtQuick 2.7
import QtQuick.Controls 2.2
import Ubuntu.Components 1.3
import QtQuick.Layouts 1.3
import Qt.labs.settings 1.0

import Tasks 1.0

ApplicationWindow {
    id: root
    objectName: 'mainView'

    width: units.gu(45)
    height: units.gu(75)
    visible: true

    Tasks {
        id: tasks
        onCountChanged: {
            label.text = tasks.i_count;
        }
    }

    Component {
        id: taskDelegate
        RowLayout {
            width: parent.width
            CheckBox {

            }
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Label {
                    text: model.name
                }
            }
            // TODO progress bar mapped to progress and steps properties
        }
    }

    Page {
        anchors.fill: parent

        header: PageHeader {
            id: header
            title: i18n.tr('Task Progress')
        }

        ColumnLayout {
            spacing: units.gu(2)
            anchors {
                margins: units.gu(2)
                top: header.bottom
                left: parent.left
                right: parent.right
                bottom: parent.bottom
            }

            Item {
                Layout.fillHeight: true
            }

            ListView {
                anchors.fill: parent
                model: tasks
                delegate: taskDelegate

                Component.onCompleted: {
                    // for (var prop in tasks) {
                    //     print(prop += " (" + typeof(tasks[prop]) + ") = " + tasks[prop]);
                    // }
                }
            }

            Label {
                id: label
                text: i18n.tr('Press the button below!')
            }

            Button {
                text: i18n.tr('Add new task')
                onClicked: {
                    tasks.add_dummy()
                }
            }

            Item {
                Layout.fillHeight: true
            }
        }
    }
}
