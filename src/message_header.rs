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
pub struct MessageHeader {
    /// Message type.
    message_type: MessageType,

    /// Sequence number.
    sequence_number: u16,

    /// Service type.
    service_type: ServiceType
}

impl MessageHeader {
    /// Create a new MessageHeader.
    pub fn new(service_type: ServiceType, message_type: MessageType, sequence_number: u16)
    -> MessageHeader {
        MessageHeader{
            message_type:    message_type,
            sequence_number: sequence_number,
            service_type:    service_type
        }
    }

    /// Parse message header data from a slice.
    ///
    /// Returns the MessageHeader and parsed byte count.
    pub fn parse(slice: &[u8]) -> Result<(MessageHeader, usize), CalAmpError> {
        // slice index
        let mut index = 0;

        Ok((MessageHeader{
            service_type: match read_u8!(slice, index) {
                0 => ServiceType::UnacknowledgedRequest,
                1 => ServiceType::AcknowledgedRequest,
                2 => ServiceType::Response,
                x @ _ => {
                    return Err(CalAmpError::ServiceType(x));
                }
            },
            message_type: match read_u8!(slice, index) {
                0 => MessageType::Null,
                1 => MessageType::AckNak,
                2 => MessageType::EventReport,
                3 => MessageType::IdReport,
                4 => MessageType::UserData,
                5 => MessageType::ApplicationData,
                6 => MessageType::ConfigurationParameter,
                7 => MessageType::UnitRequest,
                8 => MessageType::LocateReport,
                9 => MessageType::UserDataAccumulators,
                10 => MessageType::MiniEventReport,
                11 => MessageType::MiniUser,
                x @ _ => {
                    return Err(CalAmpError::MessageType(x));
                }
            },
            sequence_number: read_u16!(slice, index)
        }, index))
    }

    /// Retrieve the message type.
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    /// Retrieve the sequence number.
    pub fn sequence_number(&self) -> u16 {
        self.sequence_number
    }

    /// Retrieve the service type.
    pub fn service_type(&self) -> &ServiceType {
        &self.service_type
    }
}

#[derive(Clone,Debug)]
pub enum MessageType {
    /// ACK/NAK message.
    AckNak,

    /// Application data message.
    ApplicationData,

    /// Configuration parameter message.
    ConfigurationParameter,

    /// Event report message.
    EventReport,

    /// ID report message.
    IdReport,

    /// Locate report message.
    LocateReport,

    /// Mini event report message.
    MiniEventReport,

    /// Mini user message.
    MiniUser,

    /// Null message.
    Null,

    /// Unit request message.
    UnitRequest,

    /// User data message.
    UserData,

    /// User data with accumulators message.
    UserDataAccumulators
}

#[derive(Clone,Debug)]
pub enum ServiceType {
    /// Acknowledged request.
    AcknowledgedRequest,

    /// Response.
    Response,

    /// Unacknowledged request.
    UnacknowledgedRequest
}
