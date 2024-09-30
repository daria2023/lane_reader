use epub::doc::EpubDoc;
use std::path::Path;
use scraper::{Html, Selector};
use crossterm::{ExecutableCommand, terminal::{self, ClearType}, cursor, event::{self, Event, KeyCode}};
use std::io::{self, Write,BufReader};
use std::time::Duration;


fn main() {
    let mut file_path = get_user_input(String::from("Please input the absolute file path: "));
    let epub_path = file_path.trim();

    let mut epub;
    if Path::new(epub_path).exists() {
        epub = EpubDoc::new(epub_path).expect("Failed to open EPUB");
        let spines = epub.spine.clone();
        for spine in spines {
            println!("Nav Id: {}", spine);
        }
        let nav_id = get_user_input(String::from("Please input the nav id: "));
        let nav_id = nav_id.trim();
        let str = epub.get_resource_str(nav_id);
        let mut cnt = String::new();
        if let Some((cnt1,_)) = str {
            cnt = parse_html(&cnt1);
            render_content_to_cmd(&cnt);
        } else {
            println!("No content found for the id.");
        }

    } else {
        println!("File not exists");
    }
}

fn get_user_input(placeholder:String) -> String {
    println!("{}",placeholder);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    println!("get the user input is {}", user_input);
    user_input
}


fn render_content_to_cmd(content: &str) {
    let mut stdout = io::stdout();

    // Split the content into lines
    let lines: Vec<&str> = content.lines().collect();

    let mut current_line = 0;
    let page_size = 10; // Display 10 lines at a time

    // Initial rendering of the first few lines
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    print_page(&lines, current_line, page_size);


    loop {

        // Handle user input to scroll
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') => {
                        if current_line > 0 {
                            current_line -= 1;  // Scroll up
                        }
                        stdout.execute(cursor::MoveTo(0, 0)).unwrap();  // Move cursor to the top-left
                        print_page(&lines, current_line, page_size);
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if current_line + page_size < lines.len() {
                            current_line += 1;  // Scroll down
                        }
                        stdout.execute(cursor::MoveTo(0, 0)).unwrap();  // Move cursor to the top-left
                        print_page(&lines, current_line, page_size);
                    }
                    KeyCode::Esc => break,  // Exit on 'Esc'
                    _ => {}
                }
            }
        }
    }
}

fn print_page(lines: &[&str], start: usize, page_size: usize) {
    let mut stdout = io::stdout();

    // Clear only the area to be printed (current content), not the entire screen
    stdout.execute(terminal::Clear(ClearType::FromCursorDown)).unwrap();

    // Print the new lines
    for i in start..std::cmp::min(start + page_size, lines.len()) {
        println!("{}", lines[i]);
    }
    stdout.flush().unwrap();  // Ensure content is displayed immediately
}


fn parse_html(html_content: &str) -> String {
    let document = Html::parse_document(html_content);

    // 创建选择器以匹配所有链接
    let selector = Selector::parse("body").unwrap();

    // Collect all text content from the <body> element(s)
    let mut extracted_text = String::new();
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        extracted_text.push_str(&text);
    }

    // Return the extracted text or an empty string if no text was found
    extracted_text
}

