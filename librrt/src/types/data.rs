/// In the data part of the tokens:
/// 01-00-00-12345-TW-RAJQFIZW-F
///                   ^^^^^^^^
/// We can have either a fixed string such as READYXXX or
/// a secret token.
pub enum Data {
	/// Variants such as Ready will be useful if the user wants to signal
	/// something to the flow. For instance, the process suggest an
	/// optional modification of the identity but the user prefers to keep
	/// as such. If the user would have changed the identity, we would find
	/// the corresponding event on chain. If the user however prefers to stick
	/// with the current version, they can confirm and send a "Ready" remark.
	Ready,

	/// During the verification process, the user sends back secret tokens
	/// on-chain. Those tokens have been sent to the various channels and proove
	/// that the user did receive them.
	Token(String),
}
