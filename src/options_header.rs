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

pub enum MobileId {
    /// No mobile ID provided.
    None,

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

pub struct OptionsHeader {
    /// Authentication details.
    authentication: Option<Vec<u8>>,

    /// Forwarding IP address.
    forwarding_ip_address: Option<[u8; 4]>,

    /// Forwarding port.
    forwarding_port: Option<u16>,

    /// Mobile ID.
    mobile_id: Option<Vec<u8>>,

    /// Redirection IP address.
    redirection_ip_address: Option<[u8; 4]>,

    /// Redirection port.
    redirection_port: Option<u16>,

    /// Routing details.
    routing: Option<Vec<u8>>
}

impl OptionsHeader {
    /// Parse options header data from a stream.
    ///
    /// Returns the OptionsHeader instance and parsed byte count.
    pub fn parse(stream: &[u8]) -> Result<(OptionsHeader, usize), CalAmpError> {
        // stream index
        let mut index = 1;

        // current item length
        let mut length = 0;

        let mut options = OptionsHeader{
            authentication: None,
            forwarding_ip_address: None,
            forwarding_port: None,
            mobile_id: None,
            redirection_ip_address: None,
            redirection_port: None,
            routing: None
        };

        if stream[0] & 1 == 1 {
            // stream contains mobile id
            length = stream[index] as usize;

            let mut v = Vec::with_capacity(length);

            index += 1;

            v.push(stream[index]);
        }

        Ok((options, index))
    }
}
