use alloc::format;
use alloc::rc::Rc;
use alloc::string::ToString;
use core::cell::RefCell;
use noli::error::Result as OsResult;
use noli::prelude::{MouseEvent, SystemApi};
use noli::println;
use noli::sys::wasabi::Api;
use noli::window::{StringSize, Window};
use saba_core::browser::Browser;
use saba_core::constants::{
    ADDRESSBAR_HEIGHT, BLACK, DARKGREY, GREY, LIGHTGREY, TOOLBAR_HEIGHT, WHITE, WINDOW_HEIGHT,
    WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH,
};
use saba_core::error::Error;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }

    fn setup_toolbar(&mut self) -> OsResult<()> {
        // ツールバーの背景の視覚を描画
        self.window
            .fill_rect(LIGHTGREY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        // ツールバーとコンテンツエリアの境目の線を描画
        self.window
            .draw_line(GREY, 0, TOOLBAR_HEIGHT, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT)?;
        self.window.draw_line(
            DARKGREY,
            0,
            TOOLBAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT + 1,
        )?;

        // アドレスバーの横に"Address:"という文字列を描画
        self.window.draw_string(
            BLACK,
            5,
            5,
            "Address:",
            StringSize::Medium,
            /*underline=*  */ false,
        )?;

        // アドレスバーの四角形を描画
        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESSBAR_HEIGHT)?;

        // アドレスバーの影の線を描画
        self.window.draw_line(GREY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window
            .draw_line(GREY, 70, 2, 70, 2 + ADDRESSBAR_HEIGHT)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;

        self.window
            .draw_line(GREY, 71, 3, 71, 1 + ADDRESSBAR_HEIGHT)?;

        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        // OsResultとResultが持つError型箱となるので変換
        if let Err(error) = self.setup_toolbar() {
            return Err(Error::InvalidUI(format!(
                "failed to initialize a toolbar with error: {:#?}",
                error
            )));
        }

        // 画面を更新
        self.window.flush();
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;

        self.run_app()?;

        Ok(())
    }

    fn run_app(&mut self) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
        }
    }

    fn handle_mouse_input(&mut self) -> Result<(), Error> {
        if let Some(MouseEvent {
            button: _button,
            position,
        }) = Api::get_mouse_cursor_info()
        {
            println!("mouse position {:?}", position)
        }

        Ok(())
    }
}
