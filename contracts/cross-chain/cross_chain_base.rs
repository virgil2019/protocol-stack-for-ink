use crate::storage_define::{AbandonedMessage, Error, Group, SentMessage};
// use ink::env::AccountId;
use ink::primitives::AccountId;
/// Trait for basic cross-chain contract
use ink::prelude::{string::String, vec::Vec};

use payload::message_define::{IContext, IReceivedMessage, ISentMessage};

#[ink::trait_definition]
pub trait CrossChainBase {
    /// Sets DAT token contract address
    #[ink(message)]
    fn set_token_contract(&mut self, token: AccountId);
    /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
    #[ink(message)]
    fn send_message(&mut self, message: ISentMessage) -> u128;
    /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
    #[ink(message)]
    fn receive_message(&mut self, message: IReceivedMessage) -> Result<(), Error>;
    /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
    #[ink(message)]
    fn abandon_message(
        &mut self,
        from_chain: String,
        id: u128,
        error_code: u16,
    ) -> Result<(), Error>;
    /// Returns messages that sent from chains `chain_names` and can be executed.
    #[ink(message)]
    fn get_executable_messages(&self, chain_names: Vec<String>) -> Vec<(String, u128)>;
    /// Triggers execution of a message sent from chain `chain_name` with id `id`
    #[ink(message)]
    fn execute_message(&mut self, chain_name: String, id: u128) -> Result<String, Error>;
    /// Returns the simplified message, this message is reset every time when a contract is called
    #[ink(message)]
    fn get_context(&self) -> Option<IContext>;
    /// Returns the number of messages sent to chain `chain_name`
    #[ink(message)]
    fn get_sent_message_number(&self, chain_name: String) -> u128;
    /// Returns the number of messages received from chain `chain_name`
    #[ink(message)]
    fn get_received_message_number(&self, chain_name: String) -> u128;
    /// Returns the message with id `id` sent to chain `chain_name`
    #[ink(message)]
    fn get_sent_message(&self, chain_name: String, id: u128) -> Result<SentMessage, Error>;
    /// Returns the message with id `id` received from chain `chain_name`
    #[ink(message)]
    fn get_received_message(
        &self,
        chain_name: String,
        id: u128,
    ) -> Result<(Vec<Group>, bool), Error>;

    /// Returns the message abandoned from chain `chain_name`
    #[ink(message)]
    fn get_abandoned_message(&self, chain_name: String) -> Vec<AbandonedMessage>;
}
