macro_rules! popup_error {
    ($error:expr, $actions:ident) => {{
        let popup = crate::popup::Popup::error($error);
        $actions.push(crate::app::Action::OpenPopup(popup));
        return Default::default();
    }};
}

macro_rules! or_popup {
    ($result:expr, $actions:ident) => {
        match $result {
            Ok(ok) => ok,
            Err(error) => crate::view::popup_error!(error, $actions),
        }
    };
}

pub(crate) use {or_popup, popup_error};
