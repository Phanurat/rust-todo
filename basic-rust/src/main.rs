use headless_chrome::Browser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. เปิด Browser (Headless: true คือไม่โชว์หน้าต่าง)
    let browser = Browser::default()?;

    // 2. สร้าง Tab ใหม่
    let tab = browser.new_tab()?;

    // 3. สั่งให้ไปที่ URL ที่ต้องการ
    println!("กำลังเปิดหน้าเว็บ...");
    tab.navigate_to("https://www.google.com")?;

    // 4. รอให้ปุ่มหรือข้อมูลปรากฏขึ้นมา
    tab.wait_for_element("input")?;

    // 5. ถ่ายรูปหน้าจอ (Screenshot)
    let png_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
        None,
        None,
        true
    )?;
    
    std::fs::write("screenshot.png", png_data)?;
    println!("บันทึกรูปภาพเรียบร้อยแล้ว!");

    Ok(())
}