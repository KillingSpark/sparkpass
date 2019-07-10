use dbus::MsgHandlerResult;

pub fn handle_session_calls(
    interface: &str,
    member: &str,
) -> Option<MsgHandlerResult> {
    match interface {
        "org.freedesktop.Secrets.Collection" => match member {
            _ => unimplemented!("Session interface"),
        },
        _ => panic!("Called session with wrong interface: {}", interface),
    }
}
