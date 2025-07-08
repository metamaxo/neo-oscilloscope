use crate::{
    backend::{self, processing::ProcessArgs},
    traits::RequestTrait,
};
use anyhow::Result;
use futures::{SinkExt as _, channel::mpsc::Sender};

use crate::backend::{settings, state};

pub struct Interface {
    pub tx: Sender<backend::Request>,
}

impl Default for Interface {
    fn default() -> Self {
        let (tx, rx) = futures::channel::mpsc::channel(100);
        backend::Backend::start(rx, tx.clone());
        Interface { tx }
    }
}

impl Interface {
    pub async fn settings<R: RequestTrait<State = settings::Settings>>(
        &mut self,
        request: R,
    ) -> Result<R::Output> {
        let (operation, receiver) = request.into_operation();
        self.tx.send(backend::Request::Settings(operation)).await?;
        Ok(receiver.await?)
    }

    pub async fn state<R: RequestTrait<State = state::State>>(
        &mut self,
        request: R,
    ) -> Result<R::Output> {
        let (operation, receiver) = request.into_operation();
        self.tx.send(backend::Request::State(operation)).await?;
        Ok(receiver.await?)
    }

    pub async fn process(&mut self, args: ProcessArgs) -> Result<()> {
        self.tx.send(backend::Request::Process(args)).await?;
        Ok(())
    }
}
