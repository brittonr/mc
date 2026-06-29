use crate::{screen, settings, ui, Game};

const ABSOLUTE_MOUSE_DELTA_THRESHOLD: f64 = 1_000.0;
const ABSOLUTE_MOUSE_SCALE: f64 = 8_000.0;
const RELATIVE_MOUSE_SCALE: f64 = 2_000.0;
const MOUSE_SCALE_EPSILON: f64 = 0.01;
const WINDOW_PITCH_EPSILON_RADIANS: f64 = 0.01;
const WINDOW_MIN_PITCH_RADIANS: f64 = std::f64::consts::FRAC_PI_2 + WINDOW_PITCH_EPSILON_RADIANS;
const WINDOW_MAX_PITCH_RADIANS: f64 =
    std::f64::consts::PI + std::f64::consts::FRAC_PI_2 - WINDOW_PITCH_EPSILON_RADIANS;

pub fn handle_window_event<T>(
    window: &winit::window::Window,
    game: &mut Game,
    ui_container: &mut ui::Container,
    event: winit::event::Event<T>,
) -> bool {
    use winit::event::*;
    let cursor_grab_mode = if cfg!(target_os = "macos") {
        winit::window::CursorGrabMode::Locked
    } else {
        winit::window::CursorGrabMode::Confined
    };
    match event {
        Event::MainEventsCleared => return true,
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion {
                delta: (xrel, yrel),
            },
            ..
        } => {
            let (rx, ry) = normalized_mouse_motion_delta(xrel, yrel, game);
            game.last_mouse_xrel = xrel;
            game.last_mouse_yrel = yrel;

            if game.focused {
                window.set_cursor_grab(cursor_grab_mode).unwrap();
                window.set_cursor_visible(false);
                if let Some(player) = game.server.player {
                    let rotation = game
                        .server
                        .entities
                        .get_component_mut(player, game.server.rotation)
                        .unwrap();
                    rotation.yaw -= rx;
                    rotation.pitch = bounded_window_pitch(rotation.pitch - ry);
                }
            } else {
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                    .unwrap();
                window.set_cursor_visible(true);
            }
        }

        Event::WindowEvent { event, .. } => match event {
            WindowEvent::ModifiersChanged(modifiers_state) => {
                game.is_ctrl_pressed = modifiers_state.ctrl();
                game.is_logo_pressed = modifiers_state.logo();
            }
            WindowEvent::CloseRequested => game.should_close = true,
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                game.dpi_factor = scale_factor;
            }

            WindowEvent::ReceivedCharacter(codepoint) => {
                if !game.focused && !game.is_ctrl_pressed && !game.is_logo_pressed {
                    ui_container.key_type(game, codepoint);
                }

                #[cfg(target_os = "macos")]
                if game.is_logo_pressed && codepoint == 'q' {
                    game.should_close = true;
                }
            }

            WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                (ElementState::Released, MouseButton::Left) => {
                    let physical_size = window.inner_size();
                    let (width, height) = physical_size.to_logical::<f64>(game.dpi_factor).into();

                    if game.server.is_connected()
                        && !game.focused
                        && !game.screen_sys.is_current_closable()
                    {
                        game.focused = true;
                        window.set_cursor_grab(cursor_grab_mode).unwrap();
                        window.set_cursor_visible(false);
                    } else if !game.focused {
                        #[cfg(not(target_arch = "wasm32"))]
                        // TODO: after Pointer Lock https://github.com/rust-windowing/winit/issues/1674
                        window
                            .set_cursor_grab(winit::window::CursorGrabMode::None)
                            .unwrap();
                        window.set_cursor_visible(true);
                        ui_container.click_at(
                            game,
                            game.last_mouse_x,
                            game.last_mouse_y,
                            width,
                            height,
                        );
                    }

                    if game.focused {
                        game.server.on_left_mouse_button(false);
                    }
                }
                (ElementState::Pressed, MouseButton::Left) => {
                    if game.focused {
                        game.server.on_left_mouse_button(true);
                    }
                }
                (ElementState::Released, MouseButton::Right) => {
                    if game.focused {
                        game.server.on_right_mouse_button(false);
                        game.server.on_right_click(&mut game.renderer);
                    }
                }
                (ElementState::Pressed, MouseButton::Right) => {
                    if game.focused {
                        game.server.on_right_mouse_button(true);
                        game.server.on_right_click(&mut game.renderer);
                    }
                }
                (_, _) => (),
            },
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = position.to_logical::<f64>(game.dpi_factor).into();
                game.last_mouse_x = x;
                game.last_mouse_y = y;

                if !game.focused {
                    let physical_size = window.inner_size();
                    let (width, height) = physical_size.to_logical::<f64>(game.dpi_factor).into();
                    ui_container.hover_at(game, x, y, width, height);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // TODO: line vs pixel delta? does pixel scrolling (e.g. touchpad) need scaling?
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        game.screen_sys.on_scroll(x.into(), y.into());
                    }
                    MouseScrollDelta::PixelDelta(position) => {
                        let (x, y) = position.into();
                        game.screen_sys.on_scroll(x, y);
                    }
                }
            }
            WindowEvent::KeyboardInput { input, .. } => {
                match (input.state, input.virtual_keycode) {
                    (ElementState::Released, Some(VirtualKeyCode::Escape)) => {
                        if game.focused {
                            window
                                .set_cursor_grab(winit::window::CursorGrabMode::None)
                                .unwrap();
                            window.set_cursor_visible(true);
                            game.focused = false;
                            game.screen_sys
                                .replace_screen(Box::new(screen::SettingsMenu::new(
                                    game.vars.clone(),
                                    true,
                                )));
                        } else if game.screen_sys.is_current_closable() {
                            window.set_cursor_grab(cursor_grab_mode).unwrap();
                            window.set_cursor_visible(false);
                            game.focused = true;
                            game.screen_sys.pop_screen();
                        }
                    }
                    (ElementState::Pressed, Some(VirtualKeyCode::Grave)) => {
                        game.console.lock().unwrap().toggle();
                    }
                    (ElementState::Pressed, Some(VirtualKeyCode::F11)) => {
                        if !game.is_fullscreen {
                            // TODO: support options for exclusive and simple fullscreen
                            // see https://docs.rs/glutin/0.22.0-alpha5/glutin/window/struct.Window.html#method.set_fullscreen
                            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                                window.current_monitor(),
                            )));
                        } else {
                            window.set_fullscreen(None);
                        }

                        game.is_fullscreen = !game.is_fullscreen;
                    }
                    (ElementState::Pressed, Some(key)) => {
                        if game.focused {
                            if let Some(steven_key) =
                                settings::Stevenkey::get_by_keycode(key, &game.vars)
                            {
                                game.server.key_press(true, steven_key);
                            }
                        } else {
                            let ctrl_pressed = game.is_ctrl_pressed || game.is_logo_pressed;
                            ui_container.key_press(game, key, true, ctrl_pressed);
                        }
                    }
                    (ElementState::Released, Some(key)) => {
                        if game.focused {
                            if let Some(steven_key) =
                                settings::Stevenkey::get_by_keycode(key, &game.vars)
                            {
                                game.server.key_press(false, steven_key);
                            }
                        } else {
                            let ctrl_pressed = game.is_ctrl_pressed;
                            ui_container.key_press(game, key, false, ctrl_pressed);
                        }
                    }
                    (_, None) => (),
                }
            }
            _ => (),
        },

        _ => (),
    }

    false
}

fn normalized_mouse_motion_delta(xrel: f64, yrel: f64, game: &Game) -> (f64, f64) {
    if xrel > ABSOLUTE_MOUSE_DELTA_THRESHOLD || yrel > ABSOLUTE_MOUSE_DELTA_THRESHOLD {
        // Heuristic for if we were passed an absolute value instead of relative
        // Workaround https://github.com/tomaka/glutin/issues/1084 MouseMotion event returns absolute instead of relative values, when running Linux in a VM
        // Note SDL2 had a hint to handle this scenario:
        // sdl2::hint::set_with_priority("SDL_MOUSE_RELATIVE_MODE_WARP", "1", &sdl2::hint::Hint::Override);
        let scale = ABSOLUTE_MOUSE_SCALE + MOUSE_SCALE_EPSILON;
        return (
            (xrel - game.last_mouse_xrel) / scale,
            (yrel - game.last_mouse_yrel) / scale,
        );
    }
    let scale = RELATIVE_MOUSE_SCALE + MOUSE_SCALE_EPSILON;
    (xrel / scale, yrel / scale)
}

fn bounded_window_pitch(pitch: f64) -> f64 {
    pitch
        .max(WINDOW_MIN_PITCH_RADIANS)
        .min(WINDOW_MAX_PITCH_RADIANS)
}
