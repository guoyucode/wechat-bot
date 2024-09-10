use anyhow::Result;
use std::time::Duration;

mod main_common;
mod wechatferry;
use wechatferry as wcf;

pub type R<V = ()> = Result<V, Box<dyn std::error::Error + Send + Sync>>;

// 53521624997@chatroom
/*

 RpcContact {
                wxid: "53521624997@chatroom",
                code: "",
                remark: "",
                name: "雪球消息-验证",
                country: "",
                province: "",
                city: "",
                gender: 0,
            },
*/
fn main() -> R<()> {
    main_common::init_lib()?;

    std::thread::sleep(Duration::from_secs(9999));

    Ok(())

}
