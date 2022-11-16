use std::net::SocketAddr;

use crate::{session::{SessionId, Application, Session}, connection::SocketSettings};

use super::{SettingOption, SessionSettingsError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConnectionType {
    Acceptor,
    Initiator,
}

impl TryFrom<&str> for ConnectionType {
    type Error = SessionSettingsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "initiator" => Ok(Self::Initiator),
            "acceptor" => Ok(Self::Acceptor),
            e => Err(SessionSettingsError::InvalidValue { setting: SettingOption::ConnectionType, value: e.into() })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SessionSetting {
    pub(crate) connection_type: ConnectionType,
    pub(crate) is_dynamic: bool,

    pub(crate) begin_string: String,
    pub(crate) sender_comp_id: String,
    pub(crate) sender_sub_id: Option<String>,
    pub(crate) sender_location_id: Option<String>,
    pub(crate) target_comp_id: String,
    pub(crate) target_sub_id: Option<String>,
    pub(crate) target_location_id: Option<String>,

    pub(crate) session_qualifier: Option<String>,
    pub(crate) default_appl_ver_id: Option<String>,

    pub(crate) non_stop_session: bool,
    pub(crate) use_local_time: bool,
    pub(crate) time_zone: Option<String>,
    pub(crate) start_day: Option<String>,
    pub(crate) end_day: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) end_time: Option<String>,
    pub(crate) milliseconds_in_time_stamp: bool,
    pub(crate) refresh_on_logon: bool,
    pub(crate) reset_on_logon: bool,
    pub(crate) reset_on_logout: bool,
    pub(crate) reset_on_disconnect: bool,
    pub(crate) send_redundant_resend_requests: bool,
    pub(crate) resend_session_level_rejects: bool,
    pub(crate) time_stamp_precision: Option<String>,
    pub(crate) enable_last_msg_seq_num_processed: bool,
    pub(crate) max_messages_in_resend_request: u32,
    pub(crate) send_logout_before_disconnect_from_timeout: bool,
    pub(crate) ignore_poss_dup_resend_requests: bool,
    pub(crate) requires_orig_sending_time: bool,

    // validation options
    pub(crate) use_data_dictionary: bool,
    pub(crate) data_dictionary: Option<String>,
    pub(crate) transport_data_dictionary: Option<String>,
    pub(crate) app_data_dictionary: Option<String>,
    pub(crate) validate_fields_out_of_order: bool,
    pub(crate) validate_fields_have_values: bool,
    pub(crate) validate_user_defined_fields: bool,
    pub(crate) validate_length_and_checksum: bool,
    pub(crate) allow_unknown_msg_fields: bool,
    pub(crate) check_latency: bool,
    pub(crate) max_latency: u32,

    pub(crate) reconnect_interval: u32,
    pub(crate) heart_bt_int: Option<u32>, //initiator only
    pub(crate) logon_timeout: u32,
    pub(crate) logout_timeout: u32,

    // TODO move this into ConnectionType
    // initiator options
    pub(crate) socket_connect_host: Option<String>,
    pub(crate) socket_connect_port: Option<u32>,
    // TODO
    // pub(crate) socket_connect_hosts: Option<String>, // initiator<n> failover
    // pub(crate) socket_connect_ports: Option<String>, // initiator<n> failover

    // acceptor options
    // TODO move this into ConnectionType
    pub(crate) socket_accept_host: Option<String>,
    pub(crate) socket_accept_port: Option<u32>,

    // storage
    pub(crate) persist_messages: bool,
    // store path
    pub(crate) file_store_path: Option<String>,

    // logging
    pub(crate) file_log_path: Option<String>,
    pub(crate) debug_file_log_path: Option<String>,

    // Socket options
    pub(crate) socket_nodelay: bool,
    pub(crate) socket_send_buffer_size: Option<String>,
    pub(crate) socket_receive_buffer_size: Option<String>,
    pub(crate) socket_send_timeout: Option<String>,
    pub(crate) socket_receive_timeout: Option<String>,

    // SSL options
    pub(crate) ssl_enable: bool,
    pub(crate) ssl_server_name: Option<String>,
    pub(crate) ssl_protocols: Option<String>,
    pub(crate) ssl_validate_certificates: Option<String>,
    pub(crate) ssl_check_certificate_revocation: Option<String>,
    pub(crate) ssl_certificate: Option<String>,
    pub(crate) ssl_certificate_password: Option<String>,
    pub(crate) ssl_require_client_certificate: Option<String>,
    pub(crate) ssl_ca_certificate: Option<String>,
}

impl SessionSetting {

    pub(crate) fn score(&self, session_id: &SessionId) -> u16 {
        let mut score = 0;
        score += match self.sender_comp_id.as_str() {
            "*" => 6,
            value if value == session_id.sender_comp_id => 7,
            _ => 0,
        };
        if let Some(sender_sub_id) = self.sender_sub_id.as_ref() {
            score += match sender_sub_id.as_str() {
                value if value == session_id.sender_sub_id => 1,
                _ => 0,
            };
        }
        if let Some(sender_loc_id) = self.sender_location_id.as_ref() {
            score += match sender_loc_id.as_str() {
                value if value == &session_id.sender_location_id => 1,
                _ => 0,
            };
        }
        score += match self.target_comp_id.as_str() {
            "*" => 6,
            value if value == session_id.target_comp_id => 7,
            _ => 0,
        };
        if let Some(target_sub_id) = self.target_sub_id.as_ref() {
            score += match target_sub_id.as_str() {
                value if value == session_id.target_sub_id => 1,
                _ => 0,
            };
        }
        if let Some(target_loc_id) = self.target_location_id.as_ref() {
            score += match target_loc_id.as_str() {
                value if value == &session_id.target_location_id => 1,
                _ => 0,
            };
        }
        if score < 12 {
            0
        } else {
            score
        }
    }

    pub(crate) fn is_dynamic(&self) -> bool {
        println!("is_dynamic: {:?} {:?} {:?}", self.is_dynamic, self.sender_comp_id, self.target_comp_id);
        self.is_dynamic
            && (
                self.sender_comp_id.as_str() == "*"
                ||
                self.target_comp_id.as_str() == "*"
            )
    }

    pub(crate) fn socket_settings(&self) -> SocketSettings {
        let is_initiator = self.connection_type == ConnectionType::Initiator;
        if is_initiator {
            println!("{:?}:{:?}", self.socket_connect_host.as_ref(), self.socket_connect_port.as_ref());
            let host = self.socket_connect_host.as_ref().expect("Some host");
            let port = self.socket_connect_port.as_ref().expect("Some port");
            SocketSettings::new(host.into(), *port)
        } else {
            let host = self.socket_accept_host.as_ref().expect("Some host");
            let port = self.socket_accept_port.as_ref().expect("Some port");
            SocketSettings::new(host.into(), *port)
        }
    }

    pub(crate) fn create(&self, app: Box<dyn Application>) -> Session {
        let is_initiator = self.connection_type == ConnectionType::Initiator;
        let session_id = self.session_id();
        let sender_default_appl_ver_id = self.default_appl_ver_id.as_ref().map(|v| v.as_str()).unwrap_or("");
        Session::builder(is_initiator, app, session_id, sender_default_appl_ver_id)
            .with_heartbeat_int(self.heart_bt_int.unwrap_or(30))
            //TODO other settings
            .build()
    }

    fn session_id(&self) -> SessionId {
        SessionId::new(
            self.begin_string.as_str(),
            self.sender_comp_id.as_str(),
            self.sender_sub_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.sender_location_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.target_comp_id.as_str(),
            self.target_sub_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.target_location_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
        )
    }

    pub(crate) fn accepts(&self, session_id: &SessionId) -> bool {
        if self.is_dynamic() {
            println!("dynamic accept: {:?} {:?} == {}", self.sender_comp_id, self.target_comp_id, session_id);
            let sender_comp_ok = match self.sender_comp_id.as_str() {
                "*" => true,
                s => s == session_id.sender_comp_id,
            };
            let target_comp_ok = match self.target_comp_id.as_str() {
                "*" => true,
                s => s == session_id.target_comp_id,
            };
            sender_comp_ok && target_comp_ok
        } else {
            println!("accept: {} == {}", self.session_id(), session_id);
            &self.session_id() == session_id
        }
    }
}