const SUPPORTED_CHAINS = {
    1: { name: 'Ethereum Mainnet', symbol: 'ETH', explorer: 'https://etherscan.io' },
    137: { name: 'Polygon', symbol: 'MATIC', explorer: 'https://polygonscan.com' },
    56: { name: 'BNB Smart Chain', symbol: 'BNB', explorer: 'https://bscscan.com' },
    42161: { name: 'Arbitrum One', symbol: 'ETH', explorer: 'https://arbiscan.io' },
    10: { name: 'Optimism', symbol: 'ETH', explorer: 'https://optimistic.etherscan.io' },
    43114: { name: 'Avalanche C-Chain', symbol: 'AVAX', explorer: 'https://snowtrace.io' },
    250: { name: 'Fantom Opera', symbol: 'FTM', explorer: 'https://ftmscan.com' },
    8453: { name: 'Base', symbol: 'ETH', explorer: 'https://basescan.org' }
};

const TOKEN_STANDARDS = {
    ERC20: { methods: ['transfer', 'approve', 'transferFrom', 'balanceOf', 'allowance'] },
    ERC721: { methods: ['transferFrom', 'safeTransferFrom', 'approve', 'setApprovalForAll'] },
    ERC1155: { methods: ['safeTransferFrom', 'safeBatchTransferFrom', 'setApprovalForAll'] }
};

const GAS_LIMITS = {
    ETH_TRANSFER: 21000,
    ERC20_TRANSFER: 65000,
    ERC20_APPROVE: 46000,
    ERC721_TRANSFER: 85000,
    CONTRACT_DEPLOY: 3000000,
    SWAP: 250000
};

const RPC_METHODS = [
    'eth_chainId',
    'eth_accounts',
    'eth_requestAccounts',
    'eth_sendTransaction',
    'eth_signTransaction',
    'eth_sign',
    'personal_sign',
    'eth_signTypedData_v4',
    'wallet_switchEthereumChain',
    'wallet_addEthereumChain',
    'wallet_watchAsset'
];

module.exports = { SUPPORTED_CHAINS, TOKEN_STANDARDS, GAS_LIMITS, RPC_METHODS };
