use super::MenuGroup;
use crate::language::LOC;
use crate::startup::get_startup_status;
use crate::{config::Config, window::WindowPosition};

use std::rc::Rc;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use tray_controls::{CheckMenuKind, MenuControl, MenuManager};
use tray_icon::menu::{
    CheckMenuItem, IsMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
};

pub static QUIT: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("quit")); // Normal
pub static ABOUT: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("about")); // Normal
pub static RESTART: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("restart")); // Normal
pub static STARTUP: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("startup")); // CheckSingle
// Normal
pub static OPEN_CONFIG: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("open_config"));
// Indicator Theme: Radio
pub static FOLLOW_INDICATOR_AREA_THEME: LazyLock<MenuId> =
    LazyLock::new(|| MenuId::new("follow_indicator_area_theme"));
pub static FOLLOW_SYSTEM_THEME: LazyLock<MenuId> =
    LazyLock::new(|| MenuId::new("follow_system_theme"));
// Monitor Radio: Radio
pub static SELECT_MOUSE_MONITOR: LazyLock<MenuId> =
    LazyLock::new(|| MenuId::new("select_mouse_monitor"));
pub static SELECT_PRIMARY_MONITOR: LazyLock<MenuId> =
    LazyLock::new(|| MenuId::new("select_primary_monitor"));
// Window Position: Radio
pub static WINDOW_POSITIONS: LazyLock<[(MenuId, WindowPosition, &str); 9]> = LazyLock::new(|| {
    [
        (
            MenuId::new("position_center"),
            WindowPosition::Center,
            LOC.position_center,
        ),
        (
            MenuId::new("position_left"),
            WindowPosition::Left,
            LOC.position_left,
        ),
        (
            MenuId::new("position_right"),
            WindowPosition::Right,
            LOC.position_right,
        ),
        (
            MenuId::new("position_top"),
            WindowPosition::Top,
            LOC.position_top,
        ),
        (
            MenuId::new("position_bottom"),
            WindowPosition::Bottom,
            LOC.position_bottom,
        ),
        (
            MenuId::new("position_top_left"),
            WindowPosition::TopLeft,
            LOC.position_top_left,
        ),
        (
            MenuId::new("position_top_right"),
            WindowPosition::TopRight,
            LOC.position_top_right,
        ),
        (
            MenuId::new("position_bottom_left"),
            WindowPosition::BottomLeft,
            LOC.position_bottom_left,
        ),
        (
            MenuId::new("position_bottom_right"),
            WindowPosition::BottomRight,
            LOC.position_bottom_right,
        ),
    ]
});

struct CreateMenuItem(MenuManager<MenuGroup>);

impl CreateMenuItem {
    fn new() -> Self {
        Self(MenuManager::new())
    }

    fn separator() -> PredefinedMenuItem {
        PredefinedMenuItem::separator()
    }

    fn quit(&mut self, text: &str) -> MenuItem {
        let menu_item = MenuItem::with_id(QUIT.clone(), text, true, None);
        self.0.insert(MenuControl::MenuItem(menu_item.clone()));
        menu_item
    }

    fn about(&mut self, text: &str) -> MenuItem {
        let menu_item = MenuItem::with_id(ABOUT.clone(), text, true, None);
        self.0.insert(MenuControl::MenuItem(menu_item.clone()));
        menu_item
    }

    fn restart(&mut self, text: &str) -> MenuItem {
        let menu_item = MenuItem::with_id(RESTART.clone(), text, true, None);
        self.0.insert(MenuControl::MenuItem(menu_item.clone()));
        menu_item
    }

    fn open_config(&mut self, text: &str) -> MenuItem {
        let menu_item = MenuItem::with_id(OPEN_CONFIG.clone(), text, true, None);
        self.0.insert(MenuControl::MenuItem(menu_item.clone()));
        menu_item
    }

    fn startup(&mut self, text: &str) -> Result<CheckMenuItem> {
        let should_startup = get_startup_status()?;
        let menu_id = STARTUP.clone();
        let check_menu_item =
            CheckMenuItem::with_id(menu_id.clone(), text, true, should_startup, None);
        self.0
            .insert(MenuControl::CheckMenu(CheckMenuKind::Separate(Rc::new(
                check_menu_item.clone(),
            ))));
        Ok(check_menu_item)
    }

    fn indicator_theme(&mut self, config: &Config) -> Result<Submenu> {
        let menu_follow_indicator_area_theme = CheckMenuItem::with_id(
            FOLLOW_INDICATOR_AREA_THEME.clone(),
            LOC.follow_indicator_area_theme,
            true,
            config.is_indicator_indicator_area_theme(),
            None,
        );

        let menu_follow_system_theme = CheckMenuItem::with_id(
            FOLLOW_SYSTEM_THEME.clone(),
            LOC.follow_system_theme,
            true,
            config.is_indicator_system_theme(),
            None,
        );

        self.0.insert(MenuControl::CheckMenu(CheckMenuKind::Radio(
            Rc::new(menu_follow_indicator_area_theme.clone()),
            Some(Rc::new(FOLLOW_INDICATOR_AREA_THEME.clone())),
            MenuGroup::RadioIndicatorIconTheme,
        )));

        self.0.insert(MenuControl::CheckMenu(CheckMenuKind::Radio(
            Rc::new(menu_follow_system_theme.clone()),
            Some(Rc::new(FOLLOW_INDICATOR_AREA_THEME.clone())),
            MenuGroup::RadioIndicatorIconTheme,
        )));

        Submenu::with_items(
            LOC.theme,
            true,
            &[
                &menu_follow_indicator_area_theme as &dyn IsMenuItem,
                &menu_follow_system_theme as &dyn IsMenuItem,
            ],
        )
        .context("Failed to apped 'Indicator Theme' to Tray Menu")
    }

    fn window_postion(&mut self, config: &Config) -> Result<Submenu> {
        let position_check_items = WINDOW_POSITIONS
            .iter()
            .map(|(menu_id, position, text)| {
                let check_menu_item = CheckMenuItem::with_id(
                    menu_id.clone(),
                    text,
                    true,
                    config.get_window_position() == *position,
                    None,
                );

                self.0.insert(MenuControl::CheckMenu(CheckMenuKind::Radio(
                    Rc::new(check_menu_item.clone()),
                    Some(Rc::new(FOLLOW_INDICATOR_AREA_THEME.clone())),
                    MenuGroup::RadioWindowPosition,
                )));

                check_menu_item
            })
            .collect::<Vec<CheckMenuItem>>();

        let position_check_refs: Vec<&dyn IsMenuItem> = position_check_items
            .iter()
            .map(|item| item as &dyn IsMenuItem)
            .collect();

        Submenu::with_items(LOC.position, true, &position_check_refs)
            .context("Failed to apped 'Window Postion' to Tray Menu")
    }

    fn select_monitor(&mut self, config: &Config) -> Result<Submenu> {
        let menu_select_primary_monitor = CheckMenuItem::with_id(
            SELECT_PRIMARY_MONITOR.clone(),
            LOC.select_primary_monitor,
            true,
            config.is_primary_monitor(),
            None,
        );

        let menu_select_mouse_monitor = CheckMenuItem::with_id(
            SELECT_MOUSE_MONITOR.clone(),
            LOC.select_mouse_monitor,
            true,
            config.is_mouse_monitor(),
            None,
        );
        self.0.insert(MenuControl::CheckMenu(CheckMenuKind::Radio(
            Rc::new(menu_select_primary_monitor.clone()),
            Some(Rc::new(SELECT_MOUSE_MONITOR.clone())),
            MenuGroup::RadioMonitorSelector,
        )));
        self.0.insert(MenuControl::CheckMenu(CheckMenuKind::Radio(
            Rc::new(menu_select_mouse_monitor.clone()),
            Some(Rc::new(SELECT_MOUSE_MONITOR.clone())),
            MenuGroup::RadioMonitorSelector,
        )));

        Submenu::with_items(
            LOC.select_monitor,
            true,
            &[
                &menu_select_primary_monitor as &dyn IsMenuItem,
                &menu_select_mouse_monitor as &dyn IsMenuItem,
            ],
        )
        .context("Failed to apped 'Select Monitor' to Tray Menu")
    }
}

pub fn create_menu(config: &Config, menu_manager: &mut MenuManager<MenuGroup>) -> Result<Menu> {
    let menu_separator = CreateMenuItem::separator();

    let mut create_menu_item = CreateMenuItem::new();

    let menu_about = create_menu_item.about(LOC.about);

    let menu_quit = create_menu_item.quit(LOC.quit);

    let menu_restart = create_menu_item.restart(LOC.restart);

    let menu_startup = create_menu_item.startup(LOC.startup)?;

    let menu_open_config = create_menu_item.open_config(LOC.open_config);

    let menu_indicator_theme = create_menu_item.indicator_theme(config)?;

    let menu_window_position = create_menu_item.window_postion(config)?;

    let menu_select_monitor = create_menu_item.select_monitor(config)?;

    *menu_manager = create_menu_item.0;

    let tray_menu = Menu::new();

    tray_menu
        .append(&menu_select_monitor)
        .context("Failed to apped 'Select Monitor up' to Tray Menu")?;
    tray_menu
        .append(&menu_window_position)
        .context("Failed to apped 'Window Postion' to Tray Menu")?;
    tray_menu
        .append(&menu_indicator_theme)
        .context("Failed to apped 'Indicator Theme' to Tray Menu")?;
    tray_menu
        .append(&menu_separator)
        .context("Failed to apped 'Separator' to Tray Menu")?;
    tray_menu
        .append(&menu_open_config)
        .context("Failed to apped 'Open Config' to Tray Menu")?;
    tray_menu
        .append(&menu_separator)
        .context("Failed to apped 'Separator' to Tray Menu")?;
    tray_menu
        .append(&menu_startup)
        .context("Failed to apped 'Satr up' to Tray Menu")?;
    tray_menu
        .append(&menu_separator)
        .context("Failed to apped 'Separator' to Tray Menu")?;
    tray_menu
        .append(&menu_restart)
        .context("Failed to apped 'Restart' to Tray Menu")?;
    tray_menu
        .append(&menu_separator)
        .context("Failed to apped 'Separator' to Tray Menu")?;
    tray_menu
        .append(&menu_about)
        .context("Failed to apped 'About' to Tray Menu")?;
    tray_menu
        .append(&menu_separator)
        .context("Failed to apped 'Separator' to Tray Menu")?;
    tray_menu
        .append(&menu_quit)
        .context("Failed to apped 'Quit' to Tray Menu")?;

    Ok(tray_menu)
}
