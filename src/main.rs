use std::fmt::Debug;

use ksni::TrayMethods;

use crate::yay::yay::get_updates;
mod yay;

#[derive(Debug)]
struct MyTray {
    updates: Vec<String>
}

impl ksni::Tray for MyTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn icon_name(&self) -> String {
        "system-software-update".into()
    }
    fn title(&self) -> String {
        "Updates available".into()
    }
    fn tool_tip(&self) -> ksni::ToolTip {
        let num = self.updates.iter().count();
        ksni::ToolTip {
            icon_name: String::from("system-software-update"),
            icon_pixmap: Vec::new(),
            title: String::from("Updates available"), 
            description: String::from(format!("{num} available")) }
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            SubMenu {
                label: "a".into(),
                submenu: vec![
                    SubMenu {
                        label: "a1".into(),
                        submenu: vec![
                            StandardItem {
                                label: "a1.1".into(),
                                ..Default::default()
                            }
                            .into(),
                            StandardItem {
                                label: "a1.2".into(),
                                ..Default::default()
                            }
                            .into(),
                        ],
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "a2".into(),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //let fetcher;
    let tray = MyTray {
        updates: get_updates()
    };
    let handle = tray.spawn().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // We can modify the tray
    // Run forever
    std::future::pending().await
}
