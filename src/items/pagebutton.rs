use crate::{
    config::AppearanceConfig,
    global::structs::{
        ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Message, Page, Task, Tasks,
    },
};
use tui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use tui_additions::framework::FrameworkItem;

/// button that on press will go to another page instead of selecting it
#[derive(Clone, Copy)]
pub enum PageButton {
    Trending,
    Popular,
    History,
    ChannelMain,
    ChannelVideos,
    ChannelPlaylists,
}

impl PageButton {
    pub fn page(&self, current_page: &Page) -> Page {
        match self {
            Self::Trending => Page::MainMenu(MainMenuPage::Trending),
            Self::Popular => Page::MainMenu(MainMenuPage::Popular),
            Self::History => Page::MainMenu(MainMenuPage::History),
            Self::ChannelMain => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Main,
            }),
            Self::ChannelVideos => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Videos,
            }),
            Self::ChannelPlaylists => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Playlists,
            }),
        }
    }
}

impl ToString for PageButton {
    fn to_string(&self) -> String {
        match self {
            Self::Popular => String::from("Popular"),
            Self::Trending => String::from("Trending"),
            Self::History => String::from("History"),
            Self::ChannelMain => String::from("Main"),
            Self::ChannelVideos => String::from("Videos"),
            Self::ChannelPlaylists => String::from("Playlists"),
        }
    }
}

impl FrameworkItem for PageButton {
    // it is basically a paragraph (text) with borders
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
        popup_render: bool,
        info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let same_page = &self.page(framework.data.state.get::<Page>().unwrap())
            == framework.data.state.get::<Page>().unwrap();

        let block = Block::default()
            .border_type(appearance.borders)
            .border_style(Style::default().fg(if info.hover {
                appearance.colors.outline_hover
            } else if same_page {
                appearance.colors.outline_secondary
            } else {
                appearance.colors.outline
            }))
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(self.to_string())
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    // when selected creates a load page task, but returns false to show that it is not being selected
    fn select(&mut self, framework: &mut tui_additions::framework::FrameworkClean) -> bool {
        let current_page = framework.data.state.get::<Page>().unwrap().clone();

        if self.page(&current_page) == current_page {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("You are already on this page"));
            return false;
        }

        let tasks = framework.data.state.get_mut::<Tasks>().unwrap();
        tasks
            .priority
            .push(Task::LoadPage(self.page(&current_page)));

        false
    }

    fn selectable(&self) -> bool {
        true
    }
}
