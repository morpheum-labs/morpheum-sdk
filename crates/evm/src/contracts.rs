//! Consolidated alloy `sol!` bindings for all EVM contracts used in Morpheum
//! cross-chain operations.
//!
//! These are the canonical definitions -- other crates (E2E harnesses, CLI,
//! relay services) should import from here rather than duplicating the ABI.

use alloy::sol;

sol! {
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
        function allowance(address owner, address spender) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);

        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
    }

    /// Official Hyperlane HypERC20Collateral ABI.
    /// Deployed via `hyperlane warp deploy` as a TransparentUpgradeableProxy.
    #[sol(rpc)]
    contract IHypERC20Collateral {
        event SentTransferRemote(
            uint32 indexed destination,
            bytes32 indexed recipient,
            uint256 amount
        );

        event ReceivedTransferRemote(
            uint32 indexed origin,
            bytes32 indexed recipient,
            uint256 amount
        );

        function transferRemote(
            uint32 destination,
            bytes32 recipient,
            uint256 amount
        ) external payable returns (bytes32 messageId);

        function balanceOf(address account) external view returns (uint256);
        function wrappedToken() external view returns (address);
        function owner() external view returns (address);
        function mailbox() external view returns (address);
        function routers(uint32 domain) external view returns (bytes32);
    }

    #[sol(rpc)]
    contract IMailbox {
        event DispatchId(bytes32 indexed messageId);

        function process(bytes calldata _metadata, bytes calldata _message) external;
    }

    /// Warp Route fee quoting — implemented by both WarpCollateral (ERC-20)
    /// and WarpNative (native ETH) contracts.
    #[sol(rpc)]
    interface IWarpFee {
        function quoteDispatch(
            uint32 destination,
            bytes32 recipient,
            uint256 amount
        ) external view returns (uint256);
    }

    /// Hyperlane MerkleTreeHook — used by relayers to read on-chain merkle state.
    #[sol(rpc)]
    interface IMerkleTreeHook {
        function root() external view returns (bytes32);
        function count() external view returns (uint32);
    }

    #[sol(rpc)]
    contract IX402Settlement {
        struct PaymentRecord {
            address payer;
            bytes32 targetAgentId;
            uint256 amount;
            address asset;
            string replyChannel;
            uint64 createdAt;
            bool settled;
            bool refunded;
        }

        function pay(
            bytes32 paymentId,
            bytes32 targetAgentId,
            uint256 amount,
            string calldata memo,
            string calldata replyChannel,
            uint256 deadline,
            uint8 v,
            bytes32 r,
            bytes32 s
        ) external payable;

        function confirmSettlement(bytes32 paymentId) external;
        function refund(bytes32 paymentId) external;
        function getPayment(bytes32 paymentId) external view returns (PaymentRecord memory);

        function getPaymentDigest(
            bytes32 paymentId,
            bytes32 targetAgentId,
            uint256 amount,
            uint256 deadline
        ) external view returns (bytes32);

        function quoteFee(
            bytes32 paymentId,
            bytes32 targetAgentId,
            uint256 amount
        ) external view returns (uint256);

        event PaymentEscrowed(
            bytes32 indexed paymentId,
            address indexed payer,
            bytes32 targetAgentId,
            uint256 amount
        );

        event SettlementConfirmed(bytes32 indexed paymentId);
        event PaymentRefunded(bytes32 indexed paymentId);
    }
}
