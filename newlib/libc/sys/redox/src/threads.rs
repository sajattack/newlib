use syscall;
use libc::{c_int, c_uint};
use core::{mem, intrinsics};

type pte_osThreadHandle = usize; //XXX

#[repr(C)]
pub enum pte_osResult {
    PTE_OS_OK = 0,
    PTE_OS_NO_RESOURCES,
    PTE_OS_GENERAL_FAILURE,
    PTE_OS_TIMEOUT,
    PTE_OS_INTERRUPTED,
    PTE_OS_INVALID_PARAM
}

use self::pte_osResult::*;

// pte_osResult pte_osInit(void)

/*
pte_osResult pte_osThreadCreate(pte_osThreadEntryPoint entryPoint,
                                int stackSize,
                                int initialPriority,
                                void *argv,
                                pte_osThreadHandle* ppte_osThreadHandle)
*/

// pte_osResult pte_osThreadStart(pte_osThreadHandle osThreadHandle)

// void pte_osThreadExit()
libc_fn!(unsafe pte_osThreadExit() {
    syscall::exit(0);
});

// pte_osResult pte_osThreadExitAndDelete(pte_osThreadHandle handle)

// pte_osResult pte_osThreadDelete(pte_osThreadHandle handle)

// pte_osResult pte_osThreadWaitForEnd(pte_osThreadHandle threadHandle)

// pte_osResult pte_osThreadCancel(pte_osThreadHandle threadHandle)

// pte_osResult pte_osThreadCheckCancel(pte_osThreadHandle threadHandle)

//void pte_osThreadSleep(unsigned int msecs)
libc_fn!(unsafe pte_osThreadSleep(msecs: c_uint) {
    let tm = syscall::TimeSpec {
        tv_sec: msecs as i64 / 1000,
        tv_nsec: (msecs as i32 % 1000) * 1000,
    };
    let mut rmtp = mem::uninitialized();
    let _ = syscall::nanosleep(&tm, &mut rmtp);
});

// pte_osThreadHandle pte_osThreadGetHandle(void)

/*
int pte_osThreadGetPriority(pte_osThreadHandle threadHandle)
pte_osResult pte_osThreadSetPriority(pte_osThreadHandle threadHandle, int newPriority)
int pte_osThreadGetMinPriority()
int pte_osThreadGetMaxPriority()
int pte_osThreadGetDefaultPriority()
*/

libc_fn!(unsafe pte_osThreadGetPriority(threadHandle: pte_osThreadHandle) -> c_int {
    // XXX Shouldn't Redox support priorities?
    1
});

libc_fn!(unsafe pte_osThreadSetPriority(threadHandle: pte_osThreadHandle, newPriority: c_int) -> pte_osResult {
    PTE_OS_OK
});


libc_fn!(unsafe pte_osThreadGetMinPriority() -> c_int {
    1
});

libc_fn!(unsafe pte_osThreadGetMaxPriority() -> c_int {
    1
});

libc_fn!(unsafe pte_osThreadGetDefaultPriority() -> c_int {
    1
});

/*
pte_osResult pte_osMutexCreate(pte_osMutexHandle *pHandle)
pte_osResult pte_osMutexDelete(pte_osMutexHandle handle)
pte_osResult pte_osMutexLock(pte_osMutexHandle handle)
pte_osResult pte_osMutexUnlock(pte_osMutexHandle handle)
*/

/*
pte_osResult pte_osSemaphoreCreate(int initialValue, pte_osSemaphoreHandle *pHandle)
pte_osResult pte_osSemaphoreDelete(pte_osSemaphoreHandle handle)
pte_osResult pte_osSemaphorePost(pte_osSemaphoreHandle handle, int count)
pte_osResult pte_osSemaphorePend(pte_osSemaphoreHandle handle, unsigned int *pTimeoutMsecs)
pte_osResult pte_osSemaphoreCancellablePend(pte_osSemaphoreHandle semHandle, unsigned int *pTimeout)
*/

/*
int pte_osAtomicExchange(int *ptarg, int val)
int pte_osAtomicCompareExchange(int *pdest, int exchange, int comp)
int pte_osAtomicExchangeAddInt(int volatile* pAddend, int value)
int pte_osAtomicExchangeAdd(int volatile* pAddend, int value)
int pte_osAtomicDecrement(int *pdest)
int pte_osAtomicIncrement(int *pdest)
*/

libc_fn!(unsafe pte_osAtomicExchange(ptarg: *mut c_int, val: c_int) -> c_int {
    intrinsics::atomic_xchg(ptarg, val)
});

libc_fn!(unsafe pte_osAtomicCompareExchange(pdest: *mut c_int, exchange: c_int, comp: c_int) -> c_int {
    intrinsics::atomic_cxchg(pdest, comp, exchange).0
});

/*
pte_osResult pte_osTlsSetValue(unsigned int index, void * value)
void * pte_osTlsGetValue(unsigned int index)
pte_osResult pte_osTlsAlloc(unsigned int *pKey)
pte_osResult pte_osTlsFree(unsigned int index)
*/
