use super::{MenuGroup, item::*};
use crate::{
    UserEvent,
    config::{CONFIG_PATH, Config},
    startup::set_startup,
};

use std::process::Command;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use tray_controls::{CheckMenuKind, MenuControl};
use winit::event_loop::EventLoopProxy;

pub struct MenuHandler {
    menu_control: MenuControl<MenuGroup>,
    config: Arc<Config>,
    proxy: EventLoopProxy<UserEvent>,
}

impl MenuHandler {
    pub fn new(
        menu_control: MenuControl<MenuGroup>,
        config: Arc<Config>,
        proxy: EventLoopProxy<UserEvent>,
    ) -> Self {
        Self {
            menu_control,
            config,
            proxy,
        }
    }

    pub fn run(&self) -> Result<()> {
        let id = self.menu_control.id();
        let config = &self.config;
        let proxy = &self.proxy;

        let menu_control = &self.menu_control;
        match menu_control {
            MenuControl::CheckMenu(check_menu_kind) => match check_menu_kind {
                CheckMenuKind::Separate(check_menu) => {
                    if id == &*STARTUP {
                        set_startup(check_menu.is_checked())
                    } else {
                        Err(anyhow!("No match single check menu: {}", id.0))
                    }
                }
                CheckMenuKind::CheckBox(_click_menu, _group) => {
                    Err(anyhow!("Not support check menu group: {}", id.0))
                }
                CheckMenuKind::Radio(checked_mwnu, _, group) => {
                    match group {
                        MenuGroup::RadioIndicatorIconTheme => {
                            let checked_mwnu_id = checked_mwnu.id();
                            if checked_mwnu_id == &*FOLLOW_INDICATOR_AREA_THEME {
                                config.set_indicator_indicator_area_theme();
                            } else if checked_mwnu_id == &*FOLLOW_SYSTEM_THEME {
                                config.set_indicator_system_theme();
                            } else {
                                // ...
                            }

                            config.save();

                            let _ = proxy
                                .send_event(UserEvent::RedrawRequested)
                                .context("Failed to send 'RedrawRequested' event");

                            Ok(())
                        }
                        MenuGroup::RadioMonitorSelector => {
                            if id == &*SELECT_MOUSE_MONITOR {
                                config.set_mouse_monitor();
                            } else if id == &*SELECT_PRIMARY_MONITOR {
                                config.set_primary_monitor();
                            } else {
                                // ...
                            }

                            config.save();

                            let _ = proxy
                                .send_event(UserEvent::MoveWindow)
                                .context("Failed to send 'Move Window' event");

                            Ok(())
                        }
                        MenuGroup::RadioWindowPosition => {
                            if let Some((_, position, _)) = WINDOW_POSITIONS
                                .iter()
                                .find(|(menu_id, _, _)| menu_id == id)
                            {
                                config.set_window_position(position.clone());
                                config.save();

                                let _ = proxy
                                    .send_event(UserEvent::MoveWindow)
                                    .context("Failed to send 'Move Window' event");
                            }
                            Ok(())
                        }
                    }
                }
            },
            MenuControl::IconMenu(_icon_menu) => Err(anyhow!("None icon menu")),
            MenuControl::MenuItem(menu_item) => {
                if menu_item.id() == &*QUIT {
                    proxy
                        .send_event(UserEvent::Exit)
                        .context("Failed to send 'Exit' event")
                } else if menu_item.id() == &*ABOUT {
                    proxy
                        .send_event(UserEvent::ShowAboutDialog)
                        .context("Failed to send 'Show About Dialog' event")
                } else if menu_item.id() == &*RESTART {
                    proxy
                        .send_event(UserEvent::Restart)
                        .context("Failed to send 'Restart' event")
                } else if menu_item.id() == &*OPEN_CONFIG {
                    Command::new("notepad.exe")
                        .arg(&*CONFIG_PATH)
                        .spawn()
                        .map(|_| ())
                        .context("Failed to open config file")
                } else {
                    Err(anyhow!("No match normal menu: {}", id.0))
                }
            }
        }
    }
}
