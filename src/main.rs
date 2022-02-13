#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
        static ref NAME: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 150), position: (300, 300), title: "点名器", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [BasicApp::file_load], OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(size: 70)]
    font_name: nwg::Font,

    #[nwg_control(text: "点名器", font: Some(&data.font_name), h_align: HTextAlign::Center)]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    name: nwg::Label,

    #[nwg_resource(size: 12)]
    font_info: nwg::Font,

    #[nwg_control(text: "", font: Some(&data.font_info), h_align: HTextAlign::Left)]
    #[nwg_layout_item(layout: grid, row: 5, col: 0)]
    info_label: nwg::Label,

    #[nwg_control(text: "点名")]
    #[nwg_layout_item(layout: grid, col: 0, row: 3, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self) {
        let mut rng = rand::thread_rng();
        let mut name_num = Uniform::from(0..NAME.lock().unwrap().len()).sample(&mut rng);
        self.name.set_text(&format!("{}",NAME.lock().unwrap().to_vec()[name_num]));
        //nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name.text()));
    }

    fn say_goodbye(&self) {
        //nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

    fn file_load(&self) {
        let file = File::open("name_list.txt");
        match file {
            Ok(ref file) => {

            },
            Err(ref err) => {
                nwg::modal_error_message(&self.window, "错误", &format!("名单文件无法加载: name_list.txt\n详细信息: {}",err));
                nwg::stop_thread_dispatch();
            }
        }
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents);
        let mut namelist: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
        NAME.lock().unwrap().extend(namelist);
        self.info_label.set_text(&format!("已载入 {} 个名字",NAME.lock().unwrap().len()));
        //nwg::modal_info_message(&self.window, "提示", &format!("载入成功{}",NAME.lock().unwrap().to_vec()[1]));
    }
}

fn main() {

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}