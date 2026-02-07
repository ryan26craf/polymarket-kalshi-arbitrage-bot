const EventEmitter = require('events');

class WalletProvider extends EventEmitter {
    constructor(options) {
        super();
        this.chainId = options.chainId || 1;
        this.accounts = [];
        this.connected = false;
    }

    async connect() {
        this.connected = true;
        this.emit('connect', { chainId: this.chainId });
        return this.accounts;
    }

    async disconnect() {
        this.connected = false;
        this.accounts = [];
        this.emit('disconnect');
    }

    async request(args) {
        const { method, params } = args;

        switch (method) {
            case 'eth_accounts':
                return this.accounts;
            case 'eth_chainId':
                return `0x${this.chainId.toString(16)}`;
            case 'eth_requestAccounts':
                return this.connect();
            case 'wallet_switchEthereumChain':
                const targetChainId = parseInt(params[0].chainId, 16);
                this.chainId = targetChainId;
                this.emit('chainChanged', `0x${targetChainId.toString(16)}`);
                return null;
            default:
                throw new Error(`Method ${method} not supported`);
        }
    }

    isConnected() {
        return this.connected;
    }
}

class HardwareWalletBridge {
    constructor(transport) {
        this.transport = transport;
        this.deviceType = null;
    }

    async detectDevice() {
        const devices = await this.transport.list();
        if (devices.length === 0) {
            throw new Error('No hardware wallet detected');
        }
        this.deviceType = devices[0].productName;
        return this.deviceType;
    }

    async getPublicKey(path) {
        const response = await this.transport.send(0xe0, 0x02, 0x00, 0x00, Buffer.from(path));
        return response.toString('hex');
    }

    async signTransaction(path, rawTx) {
        const pathBuffer = Buffer.from(path);
        const txBuffer = Buffer.from(rawTx, 'hex');
        const payload = Buffer.concat([pathBuffer, txBuffer]);
        const response = await this.transport.send(0xe0, 0x04, 0x00, 0x00, payload);
        return {
            v: response[0],
            r: response.slice(1, 33).toString('hex'),
            s: response.slice(33, 65).toString('hex')
        };
    }
}

module.exports = { WalletProvider, HardwareWalletBridge };
