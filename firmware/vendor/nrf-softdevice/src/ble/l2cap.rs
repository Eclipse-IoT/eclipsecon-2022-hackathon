//! Link-Layer Control and Adaptation Protocol

use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};
use core::{ptr, u16};

use crate::ble::*;
use crate::util::{get_union_field, Portal};
use crate::{raw, RawError, Softdevice};

#[cfg(feature = "ble-l2cap-credit-wrokaround")]
fn credit_hack_refill(conn: u16, cid: u16) {
    const CREDITS_MAX: u16 = 0xFFFF;
    const CREDITS_MIN: u16 = 1024;

    let mut credits = 0;
    let ret = unsafe { raw::sd_ble_l2cap_ch_flow_control(conn, cid, 0, &mut credits) };
    if let Err(err) = RawError::convert(ret) {
        warn!("sd_ble_l2cap_ch_flow_control credits query err {:?}", err);
        return;
    }
    trace!("sd_ble_l2cap_ch_flow_control credits={=u16:x}", credits);

    if credits > CREDITS_MIN {
        // Still enough credits, no need to refill.
        return;
    }

    debug!("refilling credits");

    let ret = unsafe { raw::sd_ble_l2cap_ch_flow_control(conn, cid, CREDITS_MAX, ptr::null_mut()) };
    if let Err(err) = RawError::convert(ret) {
        warn!("sd_ble_l2cap_ch_flow_control credits=CREDITS_MAX err {:?}", err);
        return;
    }

    let ret = unsafe { raw::sd_ble_l2cap_ch_flow_control(conn, cid, 0, ptr::null_mut()) };
    if let Err(err) = RawError::convert(ret) {
        warn!("sd_ble_l2cap_ch_flow_control credits=0 err {:?}", err);
    }
}

pub(crate) unsafe fn on_evt(ble_evt: *const raw::ble_evt_t) {
    let l2cap_evt = get_union_field(ble_evt, &(*ble_evt).evt.l2cap_evt);
    match (*ble_evt).header.evt_id as u32 {
        raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_CREDIT => {}
        raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_SDU_BUF_RELEASED => {
            let params = &l2cap_evt.params.ch_sdu_buf_released;
            let pkt = unwrap!(NonNull::new(params.sdu_buf.p_data));
            (unwrap!(PACKET_FREE))(pkt)
        }
        raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_TX => {
            let params = &l2cap_evt.params.tx;
            let pkt = unwrap!(NonNull::new(params.sdu_buf.p_data));
            portal(l2cap_evt.conn_handle).call(ble_evt);
            (unwrap!(PACKET_FREE))(pkt)
        }
        _ => {
            portal(l2cap_evt.conn_handle).call(ble_evt);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxError<P: Packet> {
    Disconnected,
    TxQueueFull(P),
    Raw(RawError),
}

impl<P: Packet> From<DisconnectedError> for TxError<P> {
    fn from(_err: DisconnectedError) -> Self {
        TxError::Disconnected
    }
}

impl<P: Packet> From<RawError> for TxError<P> {
    fn from(err: RawError) -> Self {
        TxError::Raw(err)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxError {
    Disconnected,
    AllocateFailed,
    Raw(RawError),
}

impl From<DisconnectedError> for RxError {
    fn from(_err: DisconnectedError) -> Self {
        RxError::Disconnected
    }
}

impl From<RawError> for RxError {
    fn from(err: RawError) -> Self {
        RxError::Raw(err)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SetupError {
    Disconnected,
    Refused,
    Raw(RawError),
}

impl From<DisconnectedError> for SetupError {
    fn from(_err: DisconnectedError) -> Self {
        SetupError::Disconnected
    }
}

impl From<RawError> for SetupError {
    fn from(err: RawError) -> Self {
        SetupError::Raw(err)
    }
}

const PORTAL_NEW: Portal<*const raw::ble_evt_t> = Portal::new();
static PORTALS: [Portal<*const raw::ble_evt_t>; CONNS_MAX] = [PORTAL_NEW; CONNS_MAX];
pub(crate) fn portal(conn_handle: u16) -> &'static Portal<*const raw::ble_evt_t> {
    &PORTALS[conn_handle as usize]
}

pub trait Packet: Sized {
    const MTU: usize;
    fn allocate() -> Option<NonNull<u8>>;
    fn into_raw_parts(self) -> (NonNull<u8>, usize);
    unsafe fn from_raw_parts(ptr: NonNull<u8>, len: usize) -> Self;
}

pub struct L2cap<P: Packet> {
    _private: PhantomData<*mut P>,
}

static IS_INIT: AtomicBool = AtomicBool::new(false);
static mut PACKET_FREE: Option<unsafe fn(NonNull<u8>)> = None;

impl<P: Packet> L2cap<P> {
    pub fn init(_sd: &Softdevice) -> Self {
        if IS_INIT
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            panic!("L2cap::init() called multiple times.")
        }

        unsafe {
            PACKET_FREE = Some(|ptr| {
                P::from_raw_parts(ptr, 0);
                // create Packet from pointer, will be freed on drop
            })
        }

        Self { _private: PhantomData }
    }

    pub async fn setup(&self, conn: &Connection, config: &Config, psm: u16) -> Result<Channel<P>, SetupError> {
        let sd = unsafe { Softdevice::steal() };

        let conn_handle = conn.with_state(|state| state.check_connected())?;
        let mut cid: u16 = raw::BLE_L2CAP_CID_INVALID as _;
        let params = raw::ble_l2cap_ch_setup_params_t {
            le_psm: psm,
            status: 0, // only used when responding
            rx_params: raw::ble_l2cap_ch_rx_params_t {
                rx_mps: sd.l2cap_rx_mps,
                rx_mtu: P::MTU as u16,
                sdu_buf: raw::ble_data_t {
                    len: 0,
                    p_data: ptr::null_mut(),
                },
            },
        };
        let ret = unsafe { raw::sd_ble_l2cap_ch_setup(conn_handle, &mut cid, &params) };
        if let Err(err) = RawError::convert(ret) {
            warn!("sd_ble_l2cap_ch_setup err {:?}", err);
            return Err(err.into());
        }
        debug!("cid {:?}", cid);

        portal(conn_handle)
            .wait_once(|ble_evt| unsafe {
                match (*ble_evt).header.evt_id as u32 {
                    raw::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED => return Err(SetupError::Disconnected),
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_RELEASED => {
                        // It is possible to get L2CAP_EVT_CH_RELEASED for the
                        // "half-setup" channel if the conn gets disconnected while
                        // setting it up.
                        return Err(SetupError::Disconnected);
                    }
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_SETUP => {
                        let l2cap_evt = get_union_field(ble_evt, &(*ble_evt).evt.l2cap_evt);
                        let _evt = &l2cap_evt.params.ch_setup;

                        // default is 1
                        let _ = config.credits;
                        #[cfg(not(feature = "ble-l2cap-credit-wrokaround"))]
                        if config.credits != 1 {
                            let ret =
                                raw::sd_ble_l2cap_ch_flow_control(conn_handle, cid, config.credits, ptr::null_mut());
                            if let Err(err) = RawError::convert(ret) {
                                warn!("sd_ble_l2cap_ch_flow_control err {:?}", err);
                                return Err(err.into());
                            }
                        }

                        Ok(Channel {
                            conn: conn.clone(),
                            cid,
                            _private: PhantomData,
                        })
                    }
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_SETUP_REFUSED => {
                        let l2cap_evt = get_union_field(ble_evt, &(*ble_evt).evt.l2cap_evt);
                        let _evt = &l2cap_evt.params.ch_setup_refused;
                        Err(SetupError::Refused)
                    }
                    e => panic!("unexpected event {}", e),
                }
            })
            .await
    }

    pub async fn listen(&self, conn: &Connection, config: &Config, psm: u16) -> Result<Channel<P>, SetupError> {
        self.listen_with(conn, config, move |got_psm| got_psm == psm)
            .await
            .map(|(_, ch)| ch)
    }

    pub async fn listen_with(
        &self,
        conn: &Connection,
        config: &Config,
        mut accept_psm: impl FnMut(u16) -> bool,
    ) -> Result<(u16, Channel<P>), SetupError> {
        let sd = unsafe { Softdevice::steal() };
        let conn_handle = conn.with_state(|state| state.check_connected())?;

        portal(conn_handle)
            .wait_many(|ble_evt| unsafe {
                match (*ble_evt).header.evt_id as u32 {
                    raw::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED => return Some(Err(SetupError::Disconnected)),
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_SETUP_REQUEST => {
                        let l2cap_evt = get_union_field(ble_evt, &(*ble_evt).evt.l2cap_evt);
                        let evt = &l2cap_evt.params.ch_setup_request;

                        let mut cid: u16 = l2cap_evt.local_cid;
                        if accept_psm(evt.le_psm) {
                            let params = raw::ble_l2cap_ch_setup_params_t {
                                le_psm: evt.le_psm,
                                status: raw::BLE_L2CAP_CH_STATUS_CODE_SUCCESS as _,
                                rx_params: raw::ble_l2cap_ch_rx_params_t {
                                    rx_mps: sd.l2cap_rx_mps,
                                    rx_mtu: P::MTU as u16,
                                    sdu_buf: raw::ble_data_t {
                                        len: 0,
                                        p_data: ptr::null_mut(),
                                    },
                                },
                            };

                            let ret = raw::sd_ble_l2cap_ch_setup(conn_handle, &mut cid, &params);
                            if let Err(err) = RawError::convert(ret) {
                                warn!("sd_ble_l2cap_ch_setup err {:?}", err);
                                return Some(Err(err.into()));
                            }

                            // default is 1
                            let _ = config.credits;
                            #[cfg(not(feature = "ble-l2cap-credit-wrokaround"))]
                            if config.credits != 1 {
                                let ret = raw::sd_ble_l2cap_ch_flow_control(
                                    conn_handle,
                                    cid,
                                    config.credits,
                                    ptr::null_mut(),
                                );
                                if let Err(err) = RawError::convert(ret) {
                                    warn!("sd_ble_l2cap_ch_flow_control err {:?}", err);
                                    return Some(Err(err.into()));
                                }
                            }

                            Some(Ok((
                                evt.le_psm,
                                Channel {
                                    _private: PhantomData,
                                    cid,
                                    conn: conn.clone(),
                                },
                            )))
                        } else {
                            let params = raw::ble_l2cap_ch_setup_params_t {
                                le_psm: evt.le_psm,
                                status: raw::BLE_L2CAP_CH_STATUS_CODE_LE_PSM_NOT_SUPPORTED as _,
                                rx_params: mem::zeroed(),
                            };

                            let ret = raw::sd_ble_l2cap_ch_setup(conn_handle, &mut cid, &params);
                            if let Err(_err) = RawError::convert(ret) {
                                warn!("sd_ble_l2cap_ch_setup err {:?}", _err);
                            }

                            None
                        }
                    }
                    e => panic!("unexpected event {}", e),
                }
            })
            .await
    }
}

pub struct Config {
    pub credits: u16,
}

pub struct Channel<P: Packet> {
    _private: PhantomData<*mut P>,
    conn: Connection,
    cid: u16,
}

impl<P: Packet> Clone for Channel<P> {
    fn clone(&self) -> Self {
        Self {
            _private: PhantomData,
            conn: self.conn.clone(),
            cid: self.cid,
        }
    }
}

impl<P: Packet> Channel<P> {
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn try_tx(&self, sdu: P) -> Result<(), TxError<P>> {
        let conn_handle = self.conn.with_state(|s| s.check_connected())?;

        let (ptr, len) = sdu.into_raw_parts();
        assert!(len <= P::MTU);
        let data = raw::ble_data_t {
            p_data: ptr.as_ptr(),
            len: len as u16,
        };

        let ret = unsafe { raw::sd_ble_l2cap_ch_tx(conn_handle, self.cid, &data) };
        match RawError::convert(ret) {
            Err(RawError::Resources) => Err(TxError::TxQueueFull(unsafe { P::from_raw_parts(ptr, len) })),
            Err(err) => {
                warn!("sd_ble_l2cap_ch_tx err {:?}", err);
                // The SD didn't take ownership of the buffer, so it's on us to free it.
                // Reconstruct the P and let it get dropped.
                unsafe { P::from_raw_parts(ptr, len) };

                Err(err.into())
            }
            Ok(()) => Ok(()),
        }
    }

    pub async fn tx(&self, mut sdu: P) -> Result<(), TxError<P>> {
        let conn_handle = self.conn.with_state(|s| s.check_connected())?;

        loop {
            match self.try_tx(sdu) {
                Ok(()) => {
                    return Ok(());
                }
                Err(TxError::TxQueueFull(ret_sdu)) => {
                    sdu = ret_sdu;
                    portal(conn_handle)
                        .wait_once(|ble_evt| unsafe {
                            match (*ble_evt).header.evt_id as u32 {
                                raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_TX => (),
                                _ => unreachable!("Invalid event"),
                            }
                        })
                        .await;
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub async fn rx(&self) -> Result<P, RxError> {
        let conn_handle = self.conn.with_state(|s| s.check_connected())?;

        let ptr = P::allocate().ok_or(RxError::AllocateFailed)?;
        let data = raw::ble_data_t {
            p_data: ptr.as_ptr(),
            len: P::MTU as u16,
        };

        let ret = unsafe { raw::sd_ble_l2cap_ch_rx(conn_handle, self.cid, &data) };
        if let Err(err) = RawError::convert(ret) {
            warn!("sd_ble_l2cap_ch_rx err {:?}", err);
            // The SD didn't take ownership of the buffer, so it's on us to free it.
            // Reconstruct the P and let it get dropped.
            unsafe { P::from_raw_parts(ptr, 0) };
            return Err(err.into());
        }

        #[cfg(feature = "ble-l2cap-credit-wrokaround")]
        credit_hack_refill(conn_handle, self.cid);

        portal(conn_handle)
            .wait_many(|ble_evt| unsafe {
                match (*ble_evt).header.evt_id as u32 {
                    raw::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED => Some(Err(RxError::Disconnected)),
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_RELEASED => Some(Err(RxError::Disconnected)),
                    raw::BLE_L2CAP_EVTS_BLE_L2CAP_EVT_CH_RX => {
                        let l2cap_evt = get_union_field(ble_evt, &(*ble_evt).evt.l2cap_evt);
                        let evt = &l2cap_evt.params.rx;

                        let ptr = unwrap!(NonNull::new(evt.sdu_buf.p_data));
                        let len = evt.sdu_len;
                        let pkt = Packet::from_raw_parts(ptr, len as usize);
                        Some(Ok(pkt))
                    }
                    _ => None,
                }
            })
            .await
    }
}
