use crate::*;

mod command_service;

pub trait CommandService {
    /// 处理Command, 返回Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
