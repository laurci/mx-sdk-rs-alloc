use multiversx_chain_vm::mem_conv;
use multiversx_sc::api::{EndpointFinishApi, EndpointFinishApiImpl};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointFinishApi for VMHooksApi<BACKEND_TYPE> {
    type EndpointFinishApiImpl = Self;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointFinishApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn finish_slice_u8(&self, bytes: &[u8]) {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                vh.finish(offset, length);
            })
        })
    }

    fn finish_big_int_raw(&self, handle: Self::BigIntHandle) {
        self.with_vm_hooks(|vh| vh.big_int_finish_signed(handle));
    }

    fn finish_big_uint_raw(&self, handle: Self::BigIntHandle) {
        self.with_vm_hooks(|vh| vh.big_int_finish_unsigned(handle));
    }

    fn finish_managed_buffer_raw(&self, handle: Self::ManagedBufferHandle) {
        self.with_vm_hooks(|vh| vh.mbuffer_finish(handle));
    }

    fn finish_u64(&self, _value: u64) {
        unreachable!()
    }

    fn finish_i64(&self, _value: i64) {
        unreachable!()
    }
}
