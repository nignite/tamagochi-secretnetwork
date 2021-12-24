import {
  CosmWasmClient,
  Secp256k1Pen,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
  SigningCosmWasmClient,
  EnigmaUtils,
} from "secretjs";
import { cli } from "webpack";
import { config } from "./config";

const customFees = {
  upload: {
    amount: [{ amount: "2000000", denom: "uscrt" }],
    gas: "3000000",
  },
  init: {
    amount: [{ amount: "500000", denom: "uscrt" }],
    gas: "500000",
  },
  exec: {
    amount: [{ amount: "500000", denom: "uscrt" }],
    gas: "2000000",
  },
  send: {
    amount: [{ amount: "80000", denom: "uscrt" }],
    gas: "80000",
  },
};

const getSigner = async (mnemonic) => {
  const signignPen = await Secp256k1Pen.fromMnemonic(mnemonic);
  const pubKey = encodeSecp256k1Pubkey(signignPen.pubkey);
  const txEncryptionSeed = EnigmaUtils.GenerateNewSeed();
  const addr = pubkeyToAddress(pubKey, "secret");

  return {
    signignPen,
    pubKey,
    addr,
    txEncryptionSeed,
  };
};

const getClient = async () => {
  const signer = await getSigner(config.mnemonic);
  const client = new SigningCosmWasmClient(
    config.rest_api,
    signer.addr,
    (signBytes) => signer.signignPen.sign(signBytes),
    signer.txEncryptionSeed,
    customFees
  );
  return client;
};

export class SecretAPI {
  private client: SigningCosmWasmClient;
  private total_saturation_time: number = 0;
  constructor() {
    getClient().then(async (client) => {
      this.client = client;
      const time = await this.getPetInfo();
      this.total_saturation_time = time;
    });
  }
  async getPetInfo() {
    const response = await this.client.queryContractSmart(config.pet_addr, {
      pet_info: {},
    });
    return response.PetInfoResponse.total_saturation_time;
  }
  async buyFood(amount: string) {
    const msg = {
      buy_food: {},
    };
    const response = await this.client.execute(
      config.market_addr,
      msg,
      "Buying food",
      [{ denom: "uscrt", amount: amount.toString() }]
    );
  }
  async sendFood(amount: string) {
    const msg = {
      send: {
        recipient: config.pet_addr,
        amount: amount.toString(),
      },
    };
    const response = await this.client.execute(config.food_addr, msg);
    console.log(response);
  }

  async getSaturationLevel() {
    const msg = {
      last_fed: {},
    };
    const response = await this.client.queryContractSmart(config.pet_addr, msg);
    const last_fed = response.LastFedResponse.timestamp;
    const current_time = Math.floor(Date.now() / 1000);
    const remaining = current_time - last_fed;
    const percentage = Math.ceil(
      100 - Math.max((remaining / this.total_saturation_time) * 100, 0)
    );

    return percentage;
  }

  async getFoodBalance() {
    const response = await this.client.queryContractSmart(config.food_addr, {
      balance: {
        address: config.address,
        key: config.viewing_key,
      },
    });
    return response.balance.amount;
  }
}

export {};
