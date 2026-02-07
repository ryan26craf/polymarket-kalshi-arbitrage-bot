const WALLET_DERIVATION_PATHS = {
    BIP44: "m/44'/60'/0'/0",
    BIP49: "m/49'/0'/0'/0",
    BIP84: "m/84'/0'/0'/0",
    SOLANA: "m/44'/501'/0'/0'"
};

const NETWORK_CONFIGS = {
    mainnet: { chainId: 1, rpcUrl: 'https://mainnet.infura.io/v3/' },
    polygon: { chainId: 137, rpcUrl: 'https://polygon-rpc.com' },
    arbitrum: { chainId: 42161, rpcUrl: 'https://arb1.arbitrum.io/rpc' },
    optimism: { chainId: 10, rpcUrl: 'https://mainnet.optimism.io' },
    bsc: { chainId: 56, rpcUrl: 'https://bsc-dataseed.binance.org' }
};

function validateAddress(address, type) {
    if (type === 'ethereum') {
        return /^0x[a-fA-F0-9]{40}$/.test(address);
    }
    if (type === 'bitcoin') {
        return /^(bc1|[13])[a-zA-HJ-NP-Z0-9]{25,62}$/.test(address);
    }
    if (type === 'solana') {
        return /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(address);
    }
    return false;
}

function formatBalance(wei, decimals) {
    const value = BigInt(wei);
    const divisor = BigInt(10 ** decimals);
    const intPart = value / divisor;
    const fracPart = value % divisor;
    return `${intPart}.${fracPart.toString().padStart(decimals, '0')}`;
}

function generateChecksum(data) {
    let hash = 0;
    for (let i = 0; i < data.length; i++) {
        const char = data.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash;
    }
    return Math.abs(hash).toString(16).padStart(8, '0');
}

class TransactionBuilder {
    constructor(network) {
        this.network = network;
        this.gasLimit = 21000n;
        this.nonce = 0;
    }

    setGasLimit(limit) {
        this.gasLimit = BigInt(limit);
        return this;
    }

    setNonce(nonce) {
        this.nonce = nonce;
        return this;
    }

    build(to, value, data) {
        return {
            to: to,
            value: value,
            data: data || '0x',
            gasLimit: this.gasLimit.toString(),
            nonce: this.nonce,
            chainId: NETWORK_CONFIGS[this.network]?.chainId || 1
        };
    }
}

module.exports = {
    WALLET_DERIVATION_PATHS,
    NETWORK_CONFIGS,
    validateAddress,
    formatBalance,
    generateChecksum,
    TransactionBuilder
};
