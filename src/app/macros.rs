macro_rules! popup_error {
    ($error:expr, $app:ident) => {{
        let popup = $crate::popup::Popup::error($error);
        $app.popups.open(popup);
        return Default::default();
    }};
}

macro_rules! or_popup {
    ($result:expr, $app:ident) => {
        match $result {
            Ok(ok) => ok,
            Err(error) => $crate::app::macros::popup_error!(error, $app),
        }
    };
}

pub(crate) use {or_popup, popup_error};
