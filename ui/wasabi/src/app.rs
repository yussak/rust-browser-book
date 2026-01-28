use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use noli::error::Result as OsResult;
use noli::prelude::{MouseEvent, SystemApi};
use noli::println;
use noli::rect::Rect;
use noli::sys::wasabi::Api;
use noli::window::{StringSize, Window};
use saba_core::browser::Browser;
use saba_core::constants::{
    ADDRESSBAR_HEIGHT, BLACK, CONTENT_AREA_HEIGHT, CONTENT_AREA_WIDTH, DARKGREY, GREY, LIGHTGREY,
    TITLE_BAR_HEIGHT, TOOLBAR_HEIGHT, WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS,
    WINDOW_WIDTH,
};
use saba_core::error::Error;
use saba_core::http::HttpResponse;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
    input_mode: InputMode,
    input_url: String,
    cursor: Cursor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
// アプリが文字入力できる状態か
enum InputMode {
    Normal,  //文字入力できない
    Editing, // 文字入力できる
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
            input_mode: InputMode::Normal,
            input_url: String::new(),
            cursor: Cursor::new(),
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

    pub fn start(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        self.setup()?;

        self.run_app(handle_url)?;

        Ok(())
    }

    fn run_app(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
            self.handle_key_input(handle_url)?;
        }
    }

    fn handle_mouse_input(&mut self) -> Result<(), Error> {
        if let Some(MouseEvent { button, position }) = Api::get_mouse_cursor_info() {
            self.window.flush_area(self.cursor.rect());
            self.cursor.set_position(position.x, position.y);
            self.window.flush_area(self.cursor.rect());
            self.cursor.flush();

            if button.l() || button.c() || button.r() {
                // 相対位置を計算
                let relative_pos = (
                    position.x - WINDOW_INIT_X_POS,
                    position.y - WINDOW_INIT_Y_POS,
                );

                // ウィンドウの外をクリックされたときは何もしない
                if relative_pos.0 < 0
                    || relative_pos.0 > WINDOW_WIDTH
                    || relative_pos.1 < 0
                    || relative_pos.1 > WINDOW_HEIGHT
                {
                    println!("button clicked OUTSIDE window: {button:?} {position:?}");
                    return Ok(());
                }

                // ツールバーの範囲をクリックされた時、InputModeをEditingに変更
                if relative_pos.1 < TOOLBAR_HEIGHT + TITLE_BAR_HEIGHT
                    && relative_pos.1 >= TITLE_BAR_HEIGHT
                {
                    self.clear_address_bar()?;
                    self.input_url = String::new();
                    self.input_mode = InputMode::Editing;
                    println!("button clicked in toolbar: {button:?} {position:?}");
                    return Ok(());
                }

                self.input_mode = InputMode::Normal;
            }
        }

        Ok(())
    }

    fn handle_key_input(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
    ) -> Result<(), Error> {
        match self.input_mode {
            InputMode::Normal => {
                // キー入力を無視
                let _ = Api::read_key();
            }
            InputMode::Editing => {
                if let Some(c) = Api::read_key() {
                    if c == 0x0A as char {
                        // EnterキーはASCIIコードで0x0Aで表す
                        // Enterキーが押されたのでナビゲーションを開始
                        self.start_navigation(handle_url, self.input_url.clone())?;

                        self.input_url = String::new();
                        self.input_mode = InputMode::Normal;
                    } else if c == 0x7F as char || c == 0x08 as char {
                        // DeleteキーまたはBackspaceキーが押されたので最後の文字を削除
                        self.input_url.pop();
                        self.update_address_bar()?;
                    } else {
                        self.input_url.push(c);
                        self.update_address_bar()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn update_address_bar(&mut self) -> Result<(), Error> {
        // アドレスバーを白く塗る
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESSBAR_HEIGHT - 2)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        // input_urlをアドレスバーに描画
        if self
            .window
            .draw_string(BLACK, 74, 6, &self.input_url, StringSize::Medium, false)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to update an address bar".to_string(),
            ));
        }

        // アドレスバーの部分の画面を更新する
        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
            .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }

    fn clear_address_bar(&mut self) -> Result<(), Error> {
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESSBAR_HEIGHT - 2)
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
            .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }

    fn start_navigation(
        &mut self,
        handle_url: fn(String) -> Result<HttpResponse, Error>,
        destination: String,
    ) -> Result<(), Error> {
        self.clear_content_area()?;

        match handle_url(destination) {
            Ok(response) => {
                let page = self.browser.borrow().current_page();
                page.borrow_mut().receive_response(response);
            }
            Err(e) => {
                return Err(e);
            }
        }

        self.update_ui()?;

        Ok(())
    }

    fn clear_content_area(&mut self) -> Result<(), Error> {
        // コンテンツエリアを白く塗りつぶす
        if self
            .window
            .fill_rect(
                WHITE,
                0,
                TOOLBAR_HEIGHT + 2,
                CONTENT_AREA_WIDTH,
                CONTENT_AREA_HEIGHT - 2,
            )
            .is_err()
        {
            return Err(Error::InvalidUI(
                "failed to clear a content area".to_string(),
            ));
        }

        self.window.flush();

        Ok(())
    }

    fn update_ui(&mut self) -> Result<(), Error> {
        let display_items = self
            .browser
            .borrow()
            .current_page()
            .borrow()
            .display_items();

        for item in display_items {
            println!("{:?}", item);
        }

        Ok(())
    }
}
