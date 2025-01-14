// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use super::menu::{make_custom_menu_item, make_menu_item, KeyEquivalent};
use crate::{
  error::OsError,
  menu::{MenuItem, MenuType},
  platform::system_tray::SystemTray as RootSystemTray,
  platform_impl::EventLoopWindowTarget,
};
use cocoa::{
  appkit::{
    NSButton, NSEventModifierFlags, NSImage, NSMenu, NSSquareStatusItemLength, NSStatusBar,
    NSStatusItem,
  },
  base::nil,
  foundation::{NSAutoreleasePool, NSData, NSSize},
};
use objc::runtime::Object;
pub struct SystemTray {}

impl SystemTray {
  pub fn initialize<T>(
    _window_target: &EventLoopWindowTarget<T>,
    system_tray: &RootSystemTray,
  ) -> Result<(), OsError> {
    const ICON_WIDTH: f64 = 18.0;
    const ICON_HEIGHT: f64 = 18.0;
    unsafe {
      // create our system tray (status bar)
      let status_item = NSStatusBar::systemStatusBar(nil)
        .statusItemWithLength_(NSSquareStatusItemLength)
        .autorelease();

      let button = status_item.button();

      // set our icon
      let nsdata = NSData::dataWithBytes_length_(
        nil,
        system_tray.icon.as_ptr() as *const std::os::raw::c_void,
        system_tray.icon.len() as u64,
      )
      .autorelease();

      let nsimage = NSImage::initWithData_(NSImage::alloc(nil), nsdata).autorelease();
      let new_size = NSSize::new(ICON_WIDTH, ICON_HEIGHT);

      button.setImage_(nsimage);
      let _: () = msg_send![nsimage, setSize: new_size];

      let menu = NSMenu::new(nil).autorelease();

      for item in &system_tray.items {
        let item_obj: *mut Object = match item {
          MenuItem::Custom(custom_menu) => {
            // build accelerators if provided
            let mut key_equivalent = None;
            let mut accelerator_string: String;
            if let Some(accelerator) = &custom_menu.keyboard_accelerators {
              accelerator_string = accelerator.clone();
              let mut ns_modifier_flags: NSEventModifierFlags = NSEventModifierFlags::empty();

              if accelerator_string.contains("<Primary>") {
                accelerator_string = accelerator_string.replace("<Primary>", "");
                ns_modifier_flags.insert(NSEventModifierFlags::NSCommandKeyMask);
              }

              if accelerator_string.contains("<Shift>") {
                accelerator_string = accelerator_string.replace("<Shift>", "");
                ns_modifier_flags.insert(NSEventModifierFlags::NSShiftKeyMask);
              }

              if accelerator_string.contains("<Ctrl>") {
                accelerator_string = accelerator_string.replace("<Ctrl>", "");
                ns_modifier_flags.insert(NSEventModifierFlags::NSControlKeyMask);
              }

              let mut masks = None;
              if !ns_modifier_flags.is_empty() {
                masks = Some(ns_modifier_flags);
              }

              key_equivalent = Some(KeyEquivalent {
                key: accelerator_string.as_str(),
                masks,
              });
            }

            make_custom_menu_item(
              custom_menu.id,
              &custom_menu.name,
              None,
              key_equivalent,
              MenuType::SystemTray,
            )
          }
          _ => make_menu_item("Not supported", None, None, MenuType::SystemTray),
        };

        menu.addItem_(item_obj);
      }

      status_item.setMenu_(menu);
    }
    Ok(())
  }
}
