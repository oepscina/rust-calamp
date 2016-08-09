// +-----------------------------------------------------------------------------------------------+
// | Copyright 2016 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License at                                                       |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

use CalAmpError;

#[derive(Clone,Debug)]
pub enum ForwardingProtocol {
    /// TCP protocol.
    Tcp,

    /// UDP protocol.
    Udp
}

#[derive(Clone,Debug)]
pub enum ForwardingOperationType {
    /// Standard forwarding.
    Forward,

    /// Forward with lookup.
    ForwardLookup,

    /// Proxy forwarding.
    Proxy
}

#[derive(Clone,Debug)]
pub enum MobileId {
    /// Electronic serial number.
    Esn(String),

    /// International mobile equipment ID, or electronic ID of the wireless modem.
    ImeiEid(String),

    /// International mobile subscriber ID of the SIM card.
    Imsi(String),

    /// IP address.
    IpAddress(String),

    /// Phone number.
    Phone(String),

    /// User defined mobile ID.
    User(Vec<u8>)
}

#[derive(Clone)]
pub struct OptionsHeader {
    /// Authentication details.
    authentication: Option<Vec<u8>>,

    /// Forwarding IP address and port.
    forwarding: Option<(String, u16, ForwardingProtocol, ForwardingOperationType)>,

    /// Mobile ID.
    mobile_id: Option<MobileId>,

    /// Redirection IP address and port.
    redirection: Option<(String, u16)>,

    /// Routing details.
    routing: Option<Vec<u8>>
}

impl OptionsHeader {
    /// Retrieve the mobile ID.
    pub fn mobile_id(&self) -> Option<MobileId> {
        self.mobile_id.clone()
    }

    /// Parse options header data from a slice.
    ///
    /// Returns the OptionsHeader instance and parsed byte count.
    pub fn parse(slice: &[u8]) -> Result<(OptionsHeader, usize), CalAmpError> {
        // slice index
        let mut index = 0;

        // option bits
        let bits = read_u8!(slice, index);

        let mut options = OptionsHeader{
            authentication: None,
            forwarding: None,
            mobile_id: None,
            redirection: None,
            routing: None
        };

        if bits >> 7 == 0 {
            // options header is not present
            return Ok((options, index));
        }

        // bit 0: indicates a mobile id has been supplied
        if bits & 1 == 1 {
            // byte 1:          length of mobile id details
            // bytes 2..length: mobile id details
            let mut length = read_u8!(slice, index) as usize;

            if length > 0 {
                let id_bytes = read_vector!(slice, index, length);

                // bit 1: mobile id type
                if (bits >> 1) & 1 == 1 {
                    // byte 1:          length of mobile id type details
                    // bytes 2..length: mobile id type details
                    length = read_u8!(slice, index) as usize;

                    options.mobile_id = match read_u8!(slice, index) {
                        1 => {
                            // mobile id is an ESN
                            let mut id = String::with_capacity(length * 2);

                            for n in id_bytes {
                                id.push((0x30 + (n >> 4)) as char);
                                id.push((0x30 + (n & 0xF)) as char);
                            }

                            Some(MobileId::Esn(id))
                        },
                        2 => {
                            // mobile id is an IMEI or EID
                            None
                        },
                        3 => {
                            // mobile id is an IMSI
                            None
                        },
                        4 => {
                            // mobile id is user defined
                            None
                        },
                        5 => {
                            // mobile id is a phone number
                            None
                        },
                        6 => {
                            // mobile id is an ip address
                            None
                        },
                        _ => {
                            // mobile id is empty
                            None
                        }
                    }
                }
            } else {
                options.mobile_id = None;
            }
        }

        // bit 2: indicates authentication has been supplied
        if (bits >> 2) & 1 == 1 {
            // byte 1:          length of authentication details
            // bytes 2..length: authentication details
            let length = read_u8!(slice, index) as usize;

            if length > 0 {
                options.authentication = Some(read_vector!(slice, index, length));
            }
        }

        // bit 3: indicates routing has been supplied
        if (bits >> 3) & 1 == 1 {
            // byte 1:          length of routing details
            // bytes 2..length: routing details
            let length = read_u8!(slice, index) as usize;

            if length > 0 {
                options.routing = Some(read_vector!(slice, index, length));
            }
        }

        // bit 4: indicates forwarding has been supplied
        if (bits >> 4) & 1 == 1 {
            // byte 1:          length of forwarding details
            // bytes 2..length: forwarding details
            let length = read_u8!(slice, index) as usize;

            if length > 0 {
                let ip = format!("{}.{}.{}.{}", read_u8!(slice, index),
                                                read_u8!(slice, index),
                                                read_u8!(slice, index),
                                                read_u8!(slice, index));

                options.forwarding = Some((ip,
                                           // port
                                           read_u16!(slice, index),

                                           // protocol
                                           match read_u8!(slice, index) {
                                               17 => ForwardingProtocol::Udp,
                                               _ => ForwardingProtocol::Tcp
                                           },

                                           // operation type
                                           match read_u8!(slice, index) {
                                               0 => ForwardingOperationType::Forward,
                                               1 => ForwardingOperationType::Proxy,
                                               _ => ForwardingOperationType::ForwardLookup
                                           }));
            }
        }

        Ok((options, index))
    }
}
