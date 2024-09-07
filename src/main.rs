use std::fs;
use windows::{
    core::*, Foundation::Uri, Globalization::DateTimeFormatting::DateTimeFormatter,
    Web::Syndication::SyndicationClient,
};

fn main() -> windows::core::Result<()> {
    // get feed
    println!("Fetching feed...");
    let uri = Uri::CreateUri(h!(
        "https://support.microsoft.com/en-us/feed/atom/4ec863cc-2ecd-e187-6cb3-b50c6545db92"
    ))?;
    let client = SyndicationClient::new()?;
    client.SetRequestHeader(
        h!("User-Agent"),
        h!("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)"),
    )?;
    let feed = client.RetrieveFeedAsync(&uri)?.get()?;

    // create date and time formatters
    let dateformatter = DateTimeFormatter::CreateDateTimeFormatter(h!(
        "{year.full}-{month.integer(2)}-{day.integer(2)}"
    ))?;
    let timeformatter = DateTimeFormatter::CreateDateTimeFormatter(h!("longtime"))?;

    // prepare HTML document string
    println!("Creating HTML...");
    let mut htmlstring =
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><title>Windows 11 Support Feed</title><style>:root{font-family:'Segoe UI'}h2,h3,p{margin:0px}a{text-decoration:none}a:hover{text-decoration:underline}.item{margin-top:1em}.date{font-size:smaller}@media(prefers-color-scheme:dark){:root{color-scheme:dark}}</style></head><body>"#.to_owned();
    htmlstring.push_str(&format!(
        "<h2>{}</h2>Updated {} {}",
        feed.Title()?.Text()?,
        dateformatter.Format(feed.LastUpdatedTime()?)?,
        timeformatter.Format(feed.LastUpdatedTime()?)?
    ));
    for item in feed.Items()? {
        htmlstring.push_str(&format!(
            r#"<div class="item"><h3>{}</h3><p class="date">Published&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; {}&nbsp; {}<br>Last updated&nbsp; {}&nbsp; {}</p><p class="content">{}</p><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></div>"#,
            item.Title()?.Text()?,
            dateformatter.Format(item.PublishedDate()?)?,
            timeformatter.Format(item.PublishedDate()?)?,
            dateformatter.Format(item.LastUpdatedTime()?)?,
            timeformatter.Format(item.LastUpdatedTime()?)?,
            item.Content()?.Text()?,
            item.Links()?.GetAt(0)?.Uri()?.AbsoluteUri()?,
            item.Links()?.GetAt(0)?.Uri()?.AbsoluteUri()?
        ));
    }
    htmlstring.push_str("</body></html>");

    // write HTML file
    fs::write("Windows 11 Support Feed.html", htmlstring).expect("Error writing HTML file!");
    println!("Sucessfully written feed to 'Windows 11 Support Feed.html'");

    Ok(())
}
