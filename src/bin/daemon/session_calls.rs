use dbus::MsgHandlerResult;

pub fn handle_session_calls(msg: &dbus::Message, interface: &str, member: &str) -> Option<MsgHandlerResult> {
    match interface {
        "org.freedesktop.Secrets.Collection" => match member {
            _ => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(
                        &"Session interface not implemented",
                    )
                    .to_message(msg)],
                });
            }
        },
        _ => panic!("Called session with wrong interface: {}", interface),
    }
}
