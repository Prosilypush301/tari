//  Copyright 2019 The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::base_node::comms_interface::{CommsInterfaceError, NodeCommsRequest, NodeCommsResponse};
use futures::channel::oneshot::Sender as OneshotSender;
use rand::RngCore;
use serde::{Deserialize, Serialize};

pub type RequestKey = u64;

/// Generate a new random request key to uniquely identify a request and its corresponding responses.
pub fn generate_request_key<R>(rng: &mut R) -> RequestKey
where R: RngCore {
    rng.next_u64()
}

/// The WaitingRequest is used to link incoming responses to the original sent request. When enough responses have been
/// received or the request timeout has been received then the received responses are returned on the reply_tx.
#[derive(Debug)]
pub struct WaitingRequest {
    pub(crate) reply_tx: Option<OneshotSender<Result<Vec<NodeCommsResponse>, CommsInterfaceError>>>,
    pub(crate) received_responses: Vec<NodeCommsResponse>,
    pub(crate) desired_resp_count: usize,
}

/// Request type for a received BaseNodeService request.
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseNodeServiceRequest {
    pub request_key: RequestKey,
    pub request: NodeCommsRequest,
}