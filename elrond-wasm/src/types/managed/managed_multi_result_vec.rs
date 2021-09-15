use super::{ManagedBuffer, ManagedDefault, ManagedFrom, ManagedType, ManagedVec, ManagedVecItem};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, Handle, ManagedTypeApi},
    types::{ManagedArgBuffer, MultiResultVec},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};
use alloc::string::String;
use core::{marker::PhantomData, ops::Deref};

pub struct ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub(super) raw_buffers: ManagedVec<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

// impl<M, T> Deref for ManagedMultiResultVec<M, T>
// where
//     M: ManagedTypeApi,
//     T: ManagedVecItem<M>,
// {
//     type Target = ManagedVec<M, T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<M, T> ManagedType<M> for ManagedMultiResultVec<M, T>
// where
//     M: ManagedTypeApi,
// {
//     #[inline]
//     fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
//         ManagedMultiResultVec {
//             raw_buffers: Some(ManagedVec::from_raw_handle(api, raw_handle)),
//             _phantom: PhantomData,
//         }
//     }

//     #[doc(hidden)]
//     fn get_raw_handle(&self) -> Handle {
//         self.raw_buffers.get_raw_handle()
//     }

//     #[inline]
//     fn type_manager(&self) -> M {
//         self.raw_buffers.type_manager()
//     }
// }

// impl<M, T> ManagedMultiResultVec<M, T>
// where
//     M: ManagedTypeApi,
//     T: ManagedVecItem<M>,
// {
//     #[inline]
//     pub(crate) fn new_from_raw_buffer(buffer: ManagedBuffer<M>) -> Self {
//         Self(ManagedVec::new_from_raw_buffer(buffer))
//     }
// }

impl<M> From<ManagedVec<M, ManagedBuffer<M>>> for ManagedMultiResultVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(v: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        ManagedMultiResultVec {
            raw_buffers: v,
            _phantom: PhantomData,
        }
    }
}

// impl<M, T> ManagedDefault<M> for ManagedMultiResultVec<M, T>
// where
//     M: ManagedTypeApi,
//     T: ManagedVecItem<M>,
// {
//     #[inline]
//     fn managed_default(api: M) -> Self {
//         Self(ManagedVec::managed_default(api))
//     }
// }

// impl<M, T> ManagedMultiResultVec<M, T>
// where
//     M: ManagedTypeApi,
//     T: ManagedVecItem<M>,
// {
//     /// Length of the underlying buffer in bytes.

//     pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
//         self.0.slice(start_index, end_index).map(Self)
//     }

//     pub fn append_vec(&mut self, item: ManagedMultiResultVec<M, T>) {
//         self.0.append_vec(item.0);
//     }
// }

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub fn to_arg_buffer(&self) -> ManagedArgBuffer<M> {
        ManagedArgBuffer::from_raw_handle(
            self.raw_buffers.type_manager(),
            self.raw_buffers.get_raw_handle(),
        )
    }
}

impl<M> ManagedMultiResultVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<M, ManagedBuffer<M>> {
        self.raw_buffers
    }
}

impl<M, T> DynArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    // #[inline(never)]
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut raw_buffers = ManagedVec::new_empty(loader.vm_api_cast::<M>());
        while loader.has_next() {
            raw_buffers.push(ManagedBuffer::dyn_load(loader, arg_id));
        }
        ManagedMultiResultVec {
            raw_buffers,
            _phantom: PhantomData,
        }
    }
}

impl<M, T> EndpointResult for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: EndpointResult,
{
    type DecodeAs = ManagedMultiResultVec<M, T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        for elem in self.raw_buffers.into_iter() {
            elem.finish(api.clone());
        }
    }
}

impl<M, T> ContractCallArg for &ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.raw_buffers.into_iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T> TypeAbi for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> String {
        MultiResultVec::<T>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
