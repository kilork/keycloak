use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use crate::{KeycloakError, KeycloakRealmAdminMethod};

pub struct Builder<'m, M: 'm + KeycloakRealmAdminMethod> {
    method: M,
    pub(crate) args: M::Args,
    _marker: std::marker::PhantomData<&'m ()>,
}

impl<M: KeycloakRealmAdminMethod> From<M> for Builder<'_, M> {
    fn from(method: M) -> Self {
        Builder {
            method,
            args: Default::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'m, M: 'm> IntoFuture for Builder<'m, M>
where
    M: KeycloakRealmAdminMethod + Send + Sync,
{
    type Output = Result<M::Output, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'm + Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.method.opts(self.args))
    }
}
