mod debug;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use winit::raw_window_handle::HasDisplayHandle;

use ash::{
    ext::debug_utils,
    khr::{surface, swapchain as khr_swapchain},
    vk, Device, Entry, Instance,
};

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use crate::debug::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const ENABLE_VALIDATION_LAYERS: bool = true;



fn main() {
   pretty_env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct App {
    window: Option<Window>,
    instance: Option<Instance>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Vulkan tutorial with Ash")
                    .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap();

        let entry = unsafe { Entry::load().expect("Failed to create entry.") };
        let instance = create_instance(&entry, &window);

        self.window = Some(window);
        
    }

    // Called by `EventLoop::run_app` when a new event happens on the window.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}


fn create_instance(entry: &Entry, window: &Window) -> Instance {
    let app_name = CString::new("Vulkan Application").unwrap();
    let engine_name = CString::new("No Engine").unwrap();
    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name.as_c_str())
        .application_version(vk::make_api_version(0, 0, 1, 0))
        .engine_name(engine_name.as_c_str())
        .engine_version(vk::make_api_version(0, 0, 1, 0))
        .api_version(vk::make_api_version(0, 1, 0, 0));


    let extension_names =
        ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .unwrap();
    let mut extension_names = extension_names.to_vec();

    if ENABLE_VALIDATION_LAYERS {
        extension_names.push(debug_utils::NAME.as_ptr());
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        extension_names.push(ash::khr::portability_enumeration::NAME.as_ptr());
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        extension_names.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
    }

    let (_layer_names, layer_names_ptrs) = debug::get_layer_names_and_pointers();

    let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::default()
    };
    let mut debug_create_info = debug::create_debug_create_info();
    let mut instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .flags(create_flags);
    if ENABLE_VALIDATION_LAYERS {
        check_validation_layer_support(entry);
        instance_create_info = instance_create_info
            .enabled_layer_names(&layer_names_ptrs)
            .push_next(&mut debug_create_info);
    }

    unsafe { entry.create_instance(&instance_create_info, None).unwrap() }
}


