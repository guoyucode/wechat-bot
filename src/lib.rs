use anyhow::Result;
use main_common::_RPC_CONTACT;
use std::time::Duration;

mod wechatferry;
use wechatferry as wcf;
mod main_common;

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
#[no_mangle]
pub fn load_lib() -> R {
    main_common::init_lib()
}

#[no_mangle]
pub fn unload_lib() -> R {
    wechatferry::disable_listen()?;
    main_common::_S.force_drop();
    Ok(())
}

#[no_mangle]
pub fn send_text(name: &str, message: &str) -> R {
    if let Some(find) = _RPC_CONTACT.iter().find(|x| x.name == name) {
        wcf::send_text(find.wxid.clone(), message.to_string(), "".to_string())?;
        Ok(())
    }else {
         return Err(format!("not found name: {}", name))?;
    }
}

#[no_mangle]
pub fn send_files(name: &str, files: Vec<String>) -> R {
    if let Some(find) = _RPC_CONTACT.iter().find(|x| x.name == name) {
        for ele in files {
            wcf::send_file(ele.into(), find.wxid.clone())?;
        }
        Ok(())
    }else {
         return Err(format!("not found name: {}", name))?;
    }
}

#[no_mangle]
pub fn send_image(name: &str, files: Vec<String>) -> R {
    if let Some(find) = _RPC_CONTACT.iter().find(|x| x.name == name) {
        for ele in files {
            wcf::send_image(ele.into(), find.wxid.clone())?;
        }
        Ok(())
    }else {
         return Err(format!("not found name: {}", name))?;
    }
}