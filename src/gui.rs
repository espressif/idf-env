use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use native_windows_gui::{self as nwg, Window};
use once_cell::unsync::OnceCell;
use std::mem;
use std::rc::Rc;
use webview2::Controller;
use winapi::um::winuser::*;


fn get_gui_runner(_args: &str, matches: &clap::ArgMatches<'_>)  -> std::result::Result<(), clap::Error> {

    // Use an application manifest to get rid of this deprecated warning.
    #[allow(deprecated)]
        unsafe {
        nwg::set_dpi_awareness()
    };

    nwg::init().unwrap();

    let mut window = Window::default();

    Window::builder()
        .title("WebView2 - NWG")
        .size((1200, 900))
        .build(&mut window)
        .unwrap();

    let window_handle = window.handle;

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());
    let controller_clone = controller.clone();

    let result = webview2::Environment::builder().build(move |env| {
        env.unwrap()
            .create_controller(window_handle.hwnd().unwrap(), move |c| {
                let c = c.unwrap();

                unsafe {
                    let mut rect = mem::zeroed();
                    GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                    c.put_bounds(rect).unwrap();
                }

                let webview = c.get_webview().unwrap();
                // webview.navigate("http://localhost:8000").unwrap();
                // webview.navigate("file://index.html").unwrap();

                let app = include_str!("../gui/index.html");
                webview.navigate_to_string(app).unwrap();

                controller_clone.set(c).unwrap();
                Ok(())
            })
    });
    if let Err(e) = result {
        nwg::modal_fatal_message(
            &window_handle,
            "Failed to Create WebView2 Environment",
            &format!("{}", e),
        );
    }

    let window_handle = window.handle;

    // There lacks an OnWindowRestored event for SC_RESTORE in
    // native-windows-gui, so we use raw events.
    nwg::bind_raw_event_handler(&window_handle, 0xffff + 1, move |_, msg, w, _| {
        match (msg, w as usize) {
            (WM_SIZE, _) => {
                if let Some(controller) = controller.get() {
                    unsafe {
                        let mut rect = mem::zeroed();
                        GetClientRect(window_handle.hwnd().unwrap(), &mut rect);
                        controller.put_bounds(rect).unwrap();
                    }
                }
            }
            (WM_MOVE, _) => {
                if let Some(controller) = controller.get() {
                    controller.notify_parent_window_position_changed().unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_MINIMIZE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(false).unwrap();
                }
            }
            (WM_SYSCOMMAND, SC_RESTORE) => {
                if let Some(controller) = controller.get() {
                    controller.put_is_visible(true).unwrap();
                }
            }
            (WM_CLOSE, _) => nwg::stop_thread_dispatch(),
            _ => {}
        }
        None
    })
        .unwrap();

    nwg::dispatch_thread_events();
    Ok(())
}

pub fn get_gui_start_cmd<'a>() -> Command<'a, str> {
    Command::new("start")
        .description("Start GUI")

        .runner(|_args, matches|
            get_gui_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_gui_start_cmd())
        .into_cmd("gui")

        // Optionally specify a description
        .description("GUI");

    return multi_cmd;
}

