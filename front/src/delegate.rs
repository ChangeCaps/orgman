use ori::prelude::*;

use crate::AppData;

#[derive(Default)]
pub struct AppDelegate {}

impl Delegate<AppData> for AppDelegate {
    fn init(&mut self, _cx: &mut DelegateCx, _data: &mut AppData) {}

    fn event(&mut self, _cx: &mut DelegateCx, _data: &mut AppData, _event: &Event) {}
}
