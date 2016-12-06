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
use message_header::MessageType;

/// Acknowledgement message.
struct AcknowledgementMessage {
    /// Acknowledgement type.
    ack: AcknowledgementType,

    /// Application version.
    application_version: [u8; 3],

    /// Message type.
    message_type: MessageType
}

impl AcknowledgementMessage {
    /// Parse acknowledgement data from a slice.
    ///
    /// Returns the AcknowledgementMessage and parsed byte count.
    pub fn parse(slice: &[u8]) -> Result<(AcknowledgementMessage, usize), CalAmpError> {
        // slice index
        let mut index = 0;

        // message type
        let message_type = match read_u8!(slice, index) {
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
        };

        // ack
        let ack = match read_u8!(slice, index) {
            0 => AcknowledgementType::Successful,
            1 => AcknowledgementType::FailedNoReason,
            2 => AcknowledgementType::FailedMessageType,
            3 => AcknowledgementType::FailedOperation,
            4 => AcknowledgementType::FailedSerialPort,
            5 => AcknowledgementType::FailedAuthentication,
            6 => AcknowledgementType::FailedMobileId,
            7 => AcknowledgementType::FailedSequenceNumber,
            x @ _ => {
                return Err(CalAmpError::AcknowledgementType(x));
            }
        };

        // spare byte
        read_u8!(slice, index);

        // application version
        let application_version = [read_u8!(slice, index),
                                   read_u8!(slice, index),
                                   read_u8!(slice, index)];
        Ok((AcknowledgementMessage{
            ack: ack,
            application_version: application_version,
            message_type: message_type
        }, index))
    }
}

#[derive(Clone,Debug)]
pub enum AcknowledgementType {
    /// Failed ACK -- authentication failure.
    FailedAuthentication,

    /// Failed ACK -- unsupported message type.
    FailedMessageType,

    /// Failed ACK -- mobile ID lookup failure.
    FailedMobileId,

    /// Failed ACK -- no reason.
    FailedNoReason,

    /// Failed ACK -- unsupported operation.
    FailedOperation,

    /// Failed ACK -- invalid or duplicate sequence number.
    FailedSequenceNumber,

    /// Failed ACK -- unable to pass to serial port.
    FailedSerialPort,

    /// Successful ACK.
    Successful,
}
