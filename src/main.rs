use reqwest;
use scraper::{Html, Selector};
use rand::Rng;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use dialoguer::Select;

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

async fn get_attr(html: &str, css_selector: &str) -> Vec<String> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse(css_selector).unwrap();
    let elements = fragment.select(&selector);
    elements.filter_map(|element| element.value().attr("href").map(String::from))
    .map(|e| format!("https:{}", e)).collect()
}

async fn extract_content(html: &str, css_selector: &str) -> Vec<String> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse(css_selector).unwrap();
    let elements = fragment.select(&selector);
    elements.map(|element| element.inner_html()).collect()
}

fn gen_word_lenth(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn copy_words(word_list: &mut Vec<&str>, ctx: ClipboardContext) {
    let random_word_length = gen_word_lenth(6, 12);
    
    // 截取指定长度的单词
    let random_words = word_list.drain(0..random_word_length).collect::<Vec<&str>>().join(" ");
    println!("复制内容到粘贴板: {}", random_words);
    ctx.set_contents(random_words).unwrap();

}

fn prompt_continue(word_list: &mut Vec<&str>, ctx: ClipboardContext) {
    let selections = vec!["继续获取", "退出"];
    let selection = Select::new()
        .items(&selections)
        .default(0)
        .interact()
        .unwrap();
    if selection == 0 {
        copy_words(word_list, ctx);
        prompt_continue(word_list, ctx);
    } else {
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    println!("正在获取内容...");
    let url = "https://www.chinadaily.com.cn";
    let css_selector = ".tmL .twBox a";
    let html = fetch_html(url).await.unwrap();
    let hot_page_list = get_attr(&html, css_selector).await;
    
    let mut content = Vec::new();
    // parse child page
    for page in hot_page_list {
        let html = fetch_html(&page).await.unwrap();
        let css_selector = "#Content p";
        let child_page_content = extract_content(&html, css_selector).await;
        content.extend(child_page_content);
    }

    let t = content.join(" ");
    let word_list: Vec<&str> = t.split_whitespace().collect();
    // 移除非字母和数字的字符
    let mut word_list: Vec<&str> = word_list.iter().map(|word| {
        word.trim_matches(|c: char| !c.is_alphanumeric())
    }).collect();
    println!("获取成功");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    

    prompt_continue(&mut word_list, ctx);
}