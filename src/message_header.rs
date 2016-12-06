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

#[derive(Clone)]
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

impl fmt::Debug for MessageType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::AckNak => {
                write!(formatter, "MessageType::AckNak")
            },
            MessageType::ApplicationData => {
                write!(formatter, "MessageType::ApplicationData")
            },
            MessageType::ConfigurationParameter => {
                write!(formatter, "MessageType::ConfigurationParameter")
            },
            MessageType::EventReport => {
                write!(formatter, "MessageType::EventReport")
            },
            MessageType::IdReport => {
                write!(formatter, "MessageType::IdReport")
            },
            MessageType::LocateReport => {
                write!(formatter, "MessageType::LocateReport")
            },
            MessageType::MiniEventReport => {
                write!(formatter, "MessageType::MiniEventReport")
            },
            MessageType::MiniUser => {
                write!(formatter, "MessageType::MiniUser")
            },
            MessageType::Null => {
                write!(formatter, "MessageType::Null")
            },
            MessageType::UnitRequest => {
                write!(formatter, "MessageType::UnitRequest")
            },
            MessageType::UserData => {
                write!(formatter, "MessageType::UserData")
            },
            MessageType::UserDataAccumulators => {
                write!(formatter, "MessageType::UserDataAccumulators")
            }
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::AckNak => {
                write!(formatter, "AckNak")
            },
            MessageType::ApplicationData => {
                write!(formatter, "ApplicationData")
            },
            MessageType::ConfigurationParameter => {
                write!(formatter, "ConfigurationParameter")
            },
            MessageType::EventReport => {
                write!(formatter, "EventReport")
            },
            MessageType::IdReport => {
                write!(formatter, "IdReport")
            },
            MessageType::LocateReport => {
                write!(formatter, "LocateReport")
            },
            MessageType::MiniEventReport => {
                write!(formatter, "MiniEventReport")
            },
            MessageType::MiniUser => {
                write!(formatter, "MiniUser")
            },
            MessageType::Null => {
                write!(formatter, "Null")
            },
            MessageType::UnitRequest => {
                write!(formatter, "UnitRequest")
            },
            MessageType::UserData => {
                write!(formatter, "UserData")
            },
            MessageType::UserDataAccumulators => {
                write!(formatter, "UserDataAccumulators")
            }
        }
    }
}

#[derive(Clone)]
pub enum ServiceType {
    /// Acknowledged request.
    AcknowledgedRequest,

    /// Response.
    Response,

    /// Unacknowledged request.
    UnacknowledgedRequest
}

impl fmt::Debug for ServiceType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceType::AcknowledgedRequest => {
                write!(formatter, "ServiceType::AcknowledgedRequest")
            },
            ServiceType::Response => {
                write!(formatter, "ServiceType::Response")
            },
            ServiceType::UnacknowledgedRequest => {
                write!(formatter, "ServiceType::UnacknowledgedRequest")
            }
        }
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceType::AcknowledgedRequest => {
                write!(formatter, "AcknowledgedRequest")
            },
            ServiceType::Response => {
                write!(formatter, "Response")
            },
            ServiceType::UnacknowledgedRequest => {
                write!(formatter, "UnacknowledgedRequest")
            }
        }
    }
}
