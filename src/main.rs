use std::collections::HashMap;
use reqwest::header::HeaderMap;
use reqwest::header;

use i_slint_backend_winit::WinitWindowAccessor;

use tokio::{runtime::Runtime};
slint::include_modules!();
fn  main() -> Result<(), slint::PlatformError> {


     let app = App::new()?;

    let app_weak = app.as_weak();   
    app.on_mouse_move(move || {
        let app_weak = app_weak.unwrap();
        app_weak.window().with_winit_window(|win| {
            let _ = win.drag_window();
        });
    });
    let app_weak = app.as_weak();


    app.on_search(   |text| {
    let rt = tokio::runtime::Runtime::new().unwrap();


        rt.block_on(async move {
            println!("Printing in a future (L1).");
            // tokio::time::sleep(Duration::from_secs(1)).await;
            biliSearch(text.to_string()).await;
            println!("Printing in a future (L2)."); 
        });
            // Futures::executor::block_on(biliSearch(text.to_string()))
        });




    async fn biliSearch(keyword:String) -> Result<HashMap<String, String>, reqwest::Error>{
        println!("asd22");

        let url = "https://api.bilibili.com/x/web-interface/wbi/search/all/v2?keyword=".to_string() + keyword.as_str();
        let mut headers = HeaderMap::new();
        let client = reqwest::Client::new();
        let a = client.get(url)
            .header("referer", "https://www.bilibili.com")
            .header("cookie", "buvid3=0ED59B44-DBCD-D8B0-8367-728E58754C1F85632infoc; b_nut=1711329085; i-wanna-go-back=-1; b_ut=7; _uuid=81056E104D-5EB1-510EB-33108-1610371E9936195046infoc; buvid_fp=72763fa32645ca7240d063d06ebfd488;")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
            .send()
            .await?
        .text().await?;

        println!("{:?}",a);
        println!("asd33");

        Ok(reqwest::get("https://httpbin.org/ip").await?.json::<HashMap<String, String>>().await?)
    }
    println!("asd ");
    app.run()
}