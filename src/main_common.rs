use fast_able::static_type::StaticType;
use fast_able::vec::SyncVec;
use log::*;

use crate::wechatferry::proto::RpcContact;
use crate::{wechatferry, R};
use crate::wechatferry as wcf;

pub(crate) static _S: StaticType<wechatferry::CleanupHandler> = StaticType::new();

pub(crate) static _RPC_CONTACT: StaticType<SyncVec<RpcContact>> = StaticType::new();

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
pub fn init_lib() -> R {
    std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    common_uu::log4rs_mod::init_by_off("")?;
    _RPC_CONTACT.init_call(|| SyncVec::new());


    // 注册回调函数，参考 wcf::Event
    wechatferry::register_event_callback(|event| {
        info!("received event: {:#?}", event);
        // 注意，回调函数有可能是当前线程回调，也有可能是接收线程回调，不能在此函数中做复杂操作，否则可能死锁。
        // 如果希望通过回调执行复杂操作，请使用 channel 通知其他线程执行。
    });
    // auto_clean 为 true 时，返回值必须保留，否则会被自动清理
    let mut _cleanup: wechatferry::CleanupHandler = wechatferry::init(10088, true, true)?;
    _S.init_call(|| _cleanup);

    // 显式连接 command socket 后才可以调用下列测试函数
    wechatferry::connect_cmd_socket()?;

    // 测试所需变量，按实际情况设置
    // let wxid = ""; // your wxid
    // let chatroom_name = ""; // your chatroom name
    // let db_name = "MicroMsg.db";
    // let image_file = ""; // your image file here

    // 测试函数，按需启用
    println!("is_login={}", wechatferry::is_login()?);
    // println!("get_self_wx_id={:?}", wcf::get_self_wx_id()?);
    // println!("get_user_info={:?}", wcf::get_user_info()?);

    // let mut find_room_id = "".to_string();
    let contacts = wcf::get_contacts()?;
    _RPC_CONTACT.clear();
    if let Some(contacts) = contacts {
        _RPC_CONTACT.push_vec(contacts.contacts.clone().to_vec());
        // if let Some(find) = contacts.contacts.iter().find(|x| x.name.contains("雪球消息-")) {
        //     find_room_id = find.wxid.clone();
        // }
    }
    // println!("get_contacts={:#?}", wcf::get_contacts()?);

    // println!("db_names={:?}", wcf::get_db_names()?);
    // println!("db_tables={:?}", wcf::get_db_tables(db_name.into())?);
    // println!("query_all_contact_info={:?}", wcf::query_all_contact_info()?);
    // println!("query_contact_info={:?}", wcf::query_contact_info(wxid.into())?);
    // println!("query_chat_room_info={:?}", wcf::query_chat_room_info(chatroom_name.into())?);
    let sql = "SELECT ChatRoomName FROM ChatRoom";
    info!("exec_db_query={:?}", wechatferry::exec_db_query("MicroMsg.db".into(), sql.into())?);

    // if !find_room_id.is_empty() {
    //     let send_result = wcf::send_text("Are you ok? (工具转发)".into(), find_room_id.into(), "".into())?;
    //     println!("send_result={}", send_result);
    // }

    // println!("send_result={}", send_result);
    // let send_result = wcf::send_image(image_file.into(), wxid.into())?;
    // println!("send_result={}", send_result);

    wechatferry::enable_listen()?;
    info!("waiting to receive msg...");
    // std::thread::sleep(Duration::from_secs(9999999)); // check received msg in callback
    // wechatferry::disable_listen()?;

    // println!("get_msg_types={:?}", wcf::get_msg_types()?);

    // Note: these functions are not tested, but I think they should work
    // send_file, send_xml, send_emotion
    // accept_new_friend, add_chatroom_members, inv_chatroom_members, del_chatroom_members
    // decrypt_image, recv_transfer, refresh_pyq, attach_msg, get_audio_msg
    // send_rich_text, send_pat_msg, exec_ocr, forward_msg

    // wcf::uninit(); // auto_clean 为 false 时，需要显式调用 uninit()
    // println!("waiting 5s to ensure msg socket closed...");
    // std::thread::sleep(Duration::from_secs(5)); // msg socket 可能尚未关闭，等 5s
    Ok(())
}
