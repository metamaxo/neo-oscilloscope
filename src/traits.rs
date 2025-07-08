use anyhow::Result;
use futures::channel::oneshot::Sender as OneShotSender;

pub trait OperationTrait: Send + Sync {
    type State;
    fn execute(self: Box<Self>, settings: &mut Self::State) -> Result<()>;
}

pub trait RequestTrait: Send {
    type State;
    type Output: Send + 'static;
    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    );
}

pub struct Operation<S, T> {
    pub handler: Box<dyn Fn(&mut S) -> T + Send + Sync>,
    pub sender: OneShotSender<T>,
}

impl<S: 'static, T: Send + 'static> OperationTrait for Operation<S, T> {
    type State = S;
    fn execute(self: Box<Self>, state: &mut Self::State) -> Result<()> {
        let value = (self.handler)(state);
        self.sender
            .send(value)
            .map_err(|_| anyhow::anyhow!("Failed to send response"))
    }
}

impl<S: 'static, T: Send + 'static> RequestTrait for Operation<S, T> {
    type State = S;
    type Output = T;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: self.handler,
            sender: tx,
        };
        (Box::new(op), rx)
    }
}
