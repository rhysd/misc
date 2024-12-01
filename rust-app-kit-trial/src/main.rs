#![deny(unsafe_op_in_unsafe_fn)]
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{declare_class, msg_send_id, mutability, sel, ClassType, DeclaredClass};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu, NSMenuItem,
    NSStatusBar, NSStatusItem,
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSRunLoop,
    NSRunLoopCommonModes, NSString, NSTimer,
};
use std::cell::RefCell;
use std::ptr::NonNull;

#[derive(Debug, Clone, Default)]
struct Count(i32);

impl Count {
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

declare_class!(
    struct MenuItem;

    unsafe impl ClassType for MenuItem {
        type Super = NSMenuItem;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyMenuItem";
    }

    impl DeclaredClass for MenuItem {
        type Ivars = ();
    }

    unsafe impl MenuItem {
        #[method(onMenuItemClick:)]
        fn on_menu_item_click(&self,  _sender: Option<&AnyObject>) {
            println!("click!!!!!");
        }
    }
);

impl MenuItem {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(());
        let this: Retained<Self> = unsafe {
            msg_send_id![super(this), initWithTitle: ns_string!("Test"), action: Some(sel!(onMenuItemClick:)), keyEquivalent: ns_string!("")]
        };
        unsafe {
            this.setEnabled(true);
            this.setTarget(Some(&this));
        }
        this
    }
}

#[derive(Debug)]
struct IVars {
    state: RefCell<Count>,
    status: Retained<NSStatusItem>,
}

declare_class!(
    struct AppDelegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyAppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = IVars;
    }

    unsafe impl AppDelegate {
        #[method(onTimerExpired:)]
        fn on_timer_expired(&self, _: NonNull<NSTimer>) {
            let mut counter = self.ivars().state.borrow_mut();
            counter.inc();
            println!("Timer: {:?}", counter.value());

            let mtm = MainThreadMarker::from(self);
            let status = self.ivars().status.clone();
            let button = unsafe { status.button(mtm).unwrap() };
            let label = NSString::from_str(&counter.value().to_string());
            unsafe { button.setTitle(&label) }
        }
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            println!("Did finish launching!");
            let timer = unsafe {
                NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                    1.0,
                    self,
                    sel!(onTimerExpired:),
                    None,
                    true,
                )
            };
            unsafe { NSRunLoop::currentRunLoop().addTimer_forMode(&timer, NSRunLoopCommonModes) };
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            println!("Will terminate!");
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let bar = unsafe { NSStatusBar::systemStatusBar() };
        let status = unsafe { bar.statusItemWithLength(20.0) };
        unsafe {
            status.button(mtm).unwrap().setTitle(ns_string!("0"));
        }

        let menu = NSMenu::new(mtm);
        unsafe {
            menu.setTitle(ns_string!("Test Menu"));
            menu.setAutoenablesItems(false);
        }

        let item = MenuItem::new(mtm);
        menu.addItem(&item);

        unsafe {
            status.setMenu(Some(&menu));
        }

        let this = mtm.alloc();
        let this = this.set_ivars(IVars {
            state: RefCell::new(Count::default()),
            status,
        });
        unsafe { msg_send_id![super(this), init] }
    }
}

fn main() {
    let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();

    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    // configure the application delegate
    let delegate = AppDelegate::new(mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    unsafe { app.run() };
}
