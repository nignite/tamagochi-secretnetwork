import { config } from './config'
async function loadKelprOfflineSigner() {
    if (!window.getOfflineSigner || !window.kelpr) {
        alert("Please install keplr extension. ")
    }
    if (!window.keplr.experimentalSuggestChain) {
        alert("Please use latest version of the extension")
    }
    await window.keplr.experimentalSuggestChain({
        chainId: config.chainid,
        chainName: 'Secret Testnet',
        rpc: 'http://192.158.0.200:26657',
        rest: 'http://192.168.0.200:1337',
        bip44: {
            coinType: 529,
        },
        coinType: 529,
        stakeCurrency: {
            coinDenom: 'SCRT',
            coinMinimalDenom: 'uscrt',
            coinDecimals: 6,
        },
        bech32Config: {
            bech32PrefixAccAddr: 'secret',
            bech32PrefixAccPub: 'secretpub',
            bech32PrefixValAddr: 'secretvaloper',
            bech32PrefixValPub: 'secretvaloperpub',
            bech32PrefixConsAddr: 'secretvalcons',
            bech32PrefixConsPub: 'secretvalconspub',
        },
        currencies: [
            {
                coinDenom: 'SCRT',
                coinMinimalDenom: 'uscrt',
                coinDecimals: 6,
            },
        ],
        feeCurrencies: [
            {
                coinDenom: 'SCRT',
                coinMinimalDenom: 'uscrt',
                coinDecimals: 6,
            },
        ],
        gasPriceStep: {
            low: 0.1,
            average: 0.25,
            high: 0.4,
        },
        features: ['secretwasm'],
    });

    // This method will ask the user whether or not to allow access if they haven't visited this website.
    // Also, it will request user to unlock the wallet if the wallet is locked.
    // If you don't request enabling before usage, there is no guarantee that other methods will work.
    await window.keplr.enable(config.chainid);

    // @ts-ignore
    const keplrOfflineSigner = window.getOfflineSigner(config.chainid);
    const accounts = await keplrOfflineSigner.getAccounts();

    console.log(keplrOfflineSigner)
    console.log(accounts)

    return { keplrOfflineSigner, accounts };
}

export { loadKelprOfflineSigner }