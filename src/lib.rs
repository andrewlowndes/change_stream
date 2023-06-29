use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

//this stream stores a copy of the last value and only emits if the new stream value is different
#[pin_project::pin_project]
pub struct ChangeStream<S: Stream>
where
    S::Item: Clone + PartialEq,
{
    #[pin]
    stream: S,
    last_value: Option<S::Item>,
}

impl<S: Stream> ChangeStream<S>
where
    S::Item: Clone + PartialEq,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            last_value: None,
        }
    }
}

impl<S: Stream> Stream for ChangeStream<S>
where
    S::Item: Clone + PartialEq,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        //we should keep polling until we get a value that differs
        loop {
            match this.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(val)) => {
                    if let Some(last_val) = this.last_value {
                        if last_val != &val {
                            this.last_value.replace(val.clone());
                            return Poll::Ready(Some(val));
                        }
                    } else {
                        this.last_value.replace(val.clone());
                        return Poll::Ready(Some(val));
                    }
                }
                Poll::Ready(None) => {
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

pub trait StreamChanged: Stream {
    fn changed(self) -> ChangeStream<Self>
    where
        Self: Sized,
        Self::Item: Clone + PartialEq,
    {
        ChangeStream::new(self)
    }
}

impl<T: ?Sized> StreamChanged for T where T: Stream {}
