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
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
// +-----------------------------------------------------------------------------------------------+

use CalAmpError;
use std::fmt;

#[derive(Clone)]
pub enum ForwardingProtocol {
    /// TCP protocol.
    Tcp,

    /// UDP protocol.
    Udp
}

impl fmt::Debug for ForwardingProtocol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForwardingProtocol::Tcp => {
                write!(formatter, "ForwardingProtocol::Tcp")
            },
            ForwardingProtocol::Udp => {
                write!(formatter, "ForwardingProtocol::Udp")
            }
        }
    }
}

impl fmt::Display for ForwardingProtocol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForwardingProtocol::Tcp => {
                write!(formatter, "TCP")
            },
            ForwardingProtocol::Udp => {
                write!(formatter, "UDP")
            }
        }
    }
}

#[derive(Clone)]
pub enum ForwardingOperationType {
    /// Standard forwarding.
    Forward,

    /// Forward with lookup.
    ForwardLookup,

    /// Proxy forwarding.
    Proxy
}

impl fmt::Debug for ForwardingOperationType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForwardingOperationType::Forward => {
                write!(formatter, "ForwardingOperationType::Forward")
            },
            ForwardingOperationType::ForwardLookup => {
                write!(formatter, "ForwardingOperationType::ForwardLookup")
            },
            ForwardingOperationType::Proxy => {
                write!(formatter, "ForwardingOperationType::Proxy")
            }
        }
    }
}

impl fmt::Display for ForwardingOperationType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForwardingOperationType::Forward => {
                write!(formatter, "Forward")
            },
            ForwardingOperationType::ForwardLookup => {
                write!(formatter, "ForwardLookup")
            },
            ForwardingOperationType::Proxy => {
                write!(formatter, "Proxy")
            }
        }
    }
}

#[derive(Clone)]
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

impl fmt::Debug for MobileId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MobileId::Esn(ref esn) => {
                write!(formatter, "MobileId::Esn({})", esn)
            },
            MobileId::ImeiEid(ref id) => {
                write!(formatter, "MobileId::ImeiEid({})", id)
            },
            MobileId::Imsi(ref imsi) => {
                write!(formatter, "MobileId::Imsi({})", imsi)
            },
            MobileId::IpAddress(ref ip) => {
                write!(formatter, "MobileId::IpAddress({})", ip)
            },
            MobileId::Phone(ref phone) => {
                write!(formatter, "MobileId::Phone({})", phone)
            },
            MobileId::User(ref user) => {
                write!(formatter, "MobileId::User({:?})", user)
            }
        }
    }
}

impl fmt::Display for MobileId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MobileId::Esn(ref esn) => {
                write!(formatter, "{}", esn)
            },
            MobileId::ImeiEid(ref id) => {
                write!(formatter, "{}", id)
            },
            MobileId::Imsi(ref imsi) => {
                write!(formatter, "{}", imsi)
            },
            MobileId::IpAddress(ref ip) => {
                write!(formatter, "{}", ip)
            },
            MobileId::Phone(ref phone) => {
                write!(formatter, "{}", phone)
            },
            MobileId::User(ref user) => {
                write!(formatter, "{:?}", user)
            }
        }
    }
}

#[derive(Clone,Debug)]
pub struct OptionExtension {
    /// Encryption service.
    encryption_service: Option<[u8;4]>,

    /// Electronic serial number.
    esn: Option<String>,

    /// Vehicle identification number.
    vin: Option<String>
}

impl OptionExtension {
    /// Retrieve the encryption service.
    pub fn encryption_service(&self) -> &Option<[u8;4]> {
        &self.encryption_service
    }

    /// Retrieve the ESN.
    pub fn esn(&self) -> &Option<String> {
       &self.esn
    }

    /// Retrieve the VIN.
    pub fn vin(&self) -> &Option<String> {
        &self.vin
    }
}

#[derive(Clone)]
pub struct OptionsHeader {
    /// Authentication details.
    authentication: Option<Vec<u8>>,

    /// Options extension.
    extension: Option<OptionExtension>,

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
    /// Parse options header data from a slice.
    ///
    /// Returns the OptionsHeader and parsed byte count.
    pub fn parse(slice: &[u8]) -> Result<(OptionsHeader, usize), CalAmpError> {
        // slice index
        let mut index = 0;

        // option bits
        let bits = read_u8!(slice, index);

        let mut options = OptionsHeader{
            authentication: None,
            extension: None,
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
                            let mut esn = String::with_capacity(length * 2);

                            for n in id_bytes {
                                esn.push((0x30 + (n >> 4)) as char);
                                esn.push((0x30 + (n & 0xF)) as char);
                            }

                            Some(MobileId::Esn(esn))
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

        // bit 5: indicates response redirection has been supplied
        if (bits >> 5) & 1 == 1 {
            let ip = format!("{}.{}.{}.{}", read_u8!(slice, index),
                                            read_u8!(slice, index),
                                            read_u8!(slice, index),
                                            read_u8!(slice, index));

            options.redirection = Some((ip,
                                        // port
                                        read_u16!(slice, index)));
        }

        // bit 6: indicates options extension has been supplied
        if (bits >> 6) & 1 == 1 {
            // byte 1:          length of options extension (always 1 byte)
            // bytes 2..length: options extension
            let length = read_u8!(slice, index);

            if length > 1 {
                return Err(CalAmpError::OptionExtensionBitLength(length));
            };

            let mut extension = OptionExtension{ encryption_service: None,
                                                 esn: None,
                                                 vin: None};

            let extension_bits = read_u8!(slice, index);

            if (extension_bits & 1) == 1 {
                // extension bit 0: indicates ESN has been supplied
                // byte 1:          length of ESN
                // bytes 2..length: ESN
                let length  = read_u8!(slice, index) as usize;
                let mut esn = String::with_capacity(length * 2);

                for n in read_vector!(slice, index, length) {
                    esn.push((0x30 + (n >> 4)) as char);
                    esn.push((0x30 + (n & 0xF)) as char);
                }

                extension.esn = Some(esn)
            }

            if ((extension_bits >> 1) & 1) == 1 {
                // extension bit 1: indicates VIN has been supplied
                // byte 1:          length of VIN
                // bytes 2..length: VIN
                let length  = read_u8!(slice, index) as usize;

                if length != 17 {
                    return Err(CalAmpError::VinLength);
                }

                let mut vin = String::with_capacity(length);

                unsafe {
                    read_into_vector!(slice, index, length, vin.as_mut_vec());
                }

                extension.vin = Some(vin);
            }

            if ((extension_bits >> 2) & 1) == 1 {
                // extension bit 2: indicates encryption service has been supplied
                // byte 1:          length of encryption service
                // byte 2:          encryption type sub-field
                // bytes 3..length: encryption service details
                let length          = read_u8!(slice, index);
                let encryption_type = read_u8!(slice, index);

                match read_u8!(slice, index) {
                    0 => {
                        // no encryption
                    },
                    1 => {
                        // encryption is based on LMU/TTU ESN
                    },
                    2 => {
                        // encryption is based on IMEI or MEID
                    },
                    3 => {
                        // encryption is based on mobile id
                    },
                    x @ _ => {
                        return Err(CalAmpError::EncryptionType(x));
                    }
                }

                let mut random_key = [0; 4];

                read_into_array!(slice, index, random_key);

                extension.encryption_service = Some(random_key);
            }

            options.extension = Some(extension);
        }

        Ok((options, index))
    }

    /// Retrieve the authentication details.
    pub fn authentication(&self) -> &Option<Vec<u8>> {
        &self.authentication
    }

    /// Retrieve the extension details.
    pub fn extension(&self) -> &Option<OptionExtension> {
        &self.extension
    }

    /// Retrieve the forwarding details.
    pub fn forwarding(&self) -> &Option<(String, u16, ForwardingProtocol, ForwardingOperationType)> {
        &self.forwarding
    }

    /// Retrieve the mobile ID.
    pub fn mobile_id(&self) -> &Option<MobileId> {
        &self.mobile_id
    }

    /// Retrieve the redirection details.
    pub fn redirection(&self) -> &Option<(String, u16)> {
        &self.redirection
    }

    /// Retrieve the routing details.
    pub fn routing(&self) -> &Option<Vec<u8>> {
        &self.routing
    }
}
