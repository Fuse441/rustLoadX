mod config;
use crate::config::AppConfig;
use std::sync::{Arc,atomic::{AtomicUsize, Ordering}};

use std::{io, time::Duration};
use dotenv::dotenv;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
mod services;
mod produce;
use produce::service::ProduceService;
use services::utils::get_configuration;
#[tokio::main]

//-> color_eyre::Result<()>
async fn main() -> color_eyre::Result<()>  {
    dotenv().ok();
   let app_config = get_configuration().expect("Failed to load configuration");
    println!("Loaded configuration: {:?}", app_config.kafka.client_id);
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal,app_config);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    res?;
   Ok(())
}

#[derive(PartialEq)]
enum AppPage {
    Home,
    Config,
    Running,
}

struct App {
    page: AppPage,
    selected: usize,
    message_sent: Arc<AtomicUsize>,
    errors: Arc<AtomicUsize>
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,app_config:AppConfig) -> io::Result<()> {
    let message_sent = Arc::new(AtomicUsize::new(0));
    let errors = Arc::new(AtomicUsize::new(0));
    let mut app = App {
        page: AppPage::Home,
        selected: 0,
        message_sent: message_sent.clone(),
        errors: errors.clone(),
    };
    terminal.clear()?;
    loop {
    
        terminal.draw(|f| ui(f, &app,&app_config,&message_sent,&errors))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match app.page {
                    AppPage::Home => match key.code {
                        KeyCode::Up => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected < 2 {
                                app.selected += 1;
                            }
                        }
                        KeyCode::Enter => match app.selected {
                            0 => {
                             app.page = AppPage::Running;
                            
                            let client_id = app_config.kafka.client_id.clone();
                             let brokers = app_config.kafka.brokers.clone();
                            let message_sent = message_sent.clone();
                                let message_errors = errors.clone();
                                let topic = app_config.kafka.feature_load_test.topic.clone();
                             tokio::spawn(async move {
                                        let producer = rdkafka::config::ClientConfig::new()
                                            .set("bootstrap.servers", &brokers.join(","))
                                            .set("client.id", &client_id)
                                            .create::<rdkafka::producer::FutureProducer>()
                                            .expect("Failed to create producer");  
                                         // app_config.kafka.feature_load_test.topic

                            ProduceService::new(producer,10,topic).start(message_sent,message_errors).await;
                                    });
                           
                            },                             
                            1 => app.page = AppPage::Config,
                            2 => return Ok(()),
                            _ => {}
                        },
                        _ => {}
                    },
                    AppPage::Config | AppPage::Running => {
                        if key.code == KeyCode::Char('b') {
                            app.page = AppPage::Home;
                        }
                        
                    }
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App,app_config:&AppConfig,message_sent: &Arc<AtomicUsize>,error: &Arc<AtomicUsize>) {
    match app.page {
        AppPage::Home => draw_home(f, app),
        AppPage::Config => draw_config(f,&app_config),
        AppPage::Running => draw_running(f,&message_sent,&error),
    }
}

fn draw_home(f: &mut ratatui::Frame, app: &App) {
    let items = vec![
        "Start Producer",
        "Config",
        "Exit",
    ];

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if i == app.selected {
                ListItem::new(format!("> {}", item))
                    .style(Style::default().fg(Color::Yellow))
            } else {
                ListItem::new(format!("  {}", item))
            }
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().title("RustLoadX").borders(Borders::ALL));

    f.render_widget(list, f.size());
}



fn draw_config(f: &mut ratatui::Frame, app_config: &AppConfig) {
    let brokers = app_config.kafka.brokers.join(", ");

    let content = vec![
        Line::from("Config Page"),
        Line::from(""),
        Line::from(format!("Client ID: {}", app_config.kafka.client_id)),
        Line::from(format!("Brokers: {}", brokers)),
        Line::from(format!(
            "Topic: {}",
            app_config.kafka.feature_load_test.topic
        )),
        Line::from(format!(
            "Batch Size: {}",
            app_config.kafka.feature_load_test_producer.batch_size
        )),
        Line::from(format!(
            "Linger Time: {} ms",
            app_config.kafka.feature_load_test_producer.linger_time
        )),
        Line::from(""), 
        Line::from("Press 'b' to go back"),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Config").borders(Borders::ALL));

    f.render_widget(paragraph, f.size());
}


fn draw_running(
    f: &mut ratatui::Frame,
    message_sent: &Arc<AtomicUsize>,
    message_errors: &Arc<AtomicUsize>,
) {
    let sent = message_sent.load(Ordering::Relaxed);
    let error = message_errors.load(Ordering::Relaxed);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.size());

    let stats = Paragraph::new(format!("Messages Sent: {}", sent))
        .block(Block::default().title("Throughput").borders(Borders::ALL));

    let errors = Paragraph::new(format!("Errors: {}", error))
        .block(Block::default().title("Errors").borders(Borders::ALL));

    let footer = Paragraph::new("Press 'b' to go back")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(stats, chunks[0]);
    f.render_widget(errors, chunks[1]);
    f.render_widget(footer, chunks[2]);
}
