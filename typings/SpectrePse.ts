import type * as PhalaSdk from "@phala/sdk";
import type * as DevPhase from "@devphase/service";
import type { ContractCallResult, ContractQuery } from "@polkadot/api-contract/base/types";
import type { ContractCallOutcome, ContractOptions } from "@polkadot/api-contract/types";
import type { ContractExecResult } from "@polkadot/types/interfaces/contracts";
import type * as DPT from "@devphase/service/etc/typings";
import type * as PT from "@polkadot/types";
import type * as PTI from "@polkadot/types/interfaces";
import type * as PTT from "@polkadot/types/types";


/** */
/** Exported types */
/** */

export namespace InkPrimitives {
    export interface LangError {
        couldNotReadInput?: null;
        [index: string]: any;
    }

    export namespace LangError$ {
        export enum Enum {
            CouldNotReadInput = "CouldNotReadInput"
        }

        export type Human = InkPrimitives.LangError$.Enum.CouldNotReadInput & { [index: string]: any };

        export interface Codec extends PT.Enum {
            type: Enum;
            inner: PTT.Codec;
            value: PTT.Codec;
            toHuman(isExtended?: boolean): Human;
            toJSON(): LangError;
            toPrimitive(): LangError;
        }
    }
}

export namespace SpectrePse {
    export interface Error {
        unregisteredTraderAccount?: null;
        privateKeyOfThatNetworkAlreadyRegistered?: null;
        keysUnavailable?: null;
        failedToConvertPubKey?: null;
        [index: string]: any;
    }

    export interface Network {
        substrate?: null;
        ethereum?: null;
        solana?: null;
        [index: string]: any;
    }

    export interface OnchainTradingPublicKeys {
        substrate: string | number[];
        ethereum: string | number[];
        solana: string | number[];
    }

    export namespace Error$ {
        export enum Enum {
            UnregisteredTraderAccount = "UnregisteredTraderAccount",
            PrivateKeyOfThatNetworkAlreadyRegistered = "PrivateKeyOfThatNetworkAlreadyRegistered",
            KeysUnavailable = "KeysUnavailable",
            FailedToConvertPubKey = "FailedToConvertPubKey"
        }

        export type Human = SpectrePse.Error$.Enum.UnregisteredTraderAccount & { [index: string]: any }
            | SpectrePse.Error$.Enum.PrivateKeyOfThatNetworkAlreadyRegistered & { [index: string]: any }
            | SpectrePse.Error$.Enum.KeysUnavailable & { [index: string]: any }
            | SpectrePse.Error$.Enum.FailedToConvertPubKey & { [index: string]: any };

        export interface Codec extends PT.Enum {
            type: Enum;
            inner: PTT.Codec;
            value: PTT.Codec;
            toHuman(isExtended?: boolean): Human;
            toJSON(): Error;
            toPrimitive(): Error;
        }
    }

    export namespace Network$ {
        export enum Enum {
            Substrate = "Substrate",
            Ethereum = "Ethereum",
            Solana = "Solana"
        }

        export type Human = SpectrePse.Network$.Enum.Substrate & { [index: string]: any }
            | SpectrePse.Network$.Enum.Ethereum & { [index: string]: any }
            | SpectrePse.Network$.Enum.Solana & { [index: string]: any };

        export interface Codec extends PT.Enum {
            type: Enum;
            inner: PTT.Codec;
            value: PTT.Codec;
            toHuman(isExtended?: boolean): Human;
            toJSON(): Network;
            toPrimitive(): Network;
        }
    }

    export namespace OnchainTradingPublicKeys$ {
        export interface Human {
            substrate: string;
            ethereum: string;
            solana: string;
        }

        export interface Codec extends DPT.Json<SpectrePse.OnchainTradingPublicKeys, SpectrePse.OnchainTradingPublicKeys$.Human> {
            substrate: PTI.AccountId;
            ethereum: PTI.AccountId;
            solana: PTI.AccountId;
        }
    }
}

export namespace PinkExtension {
    export namespace ChainExtension {
        export type PinkExt = any;

        export namespace PinkExt$ {
            export type Enum = any;
            export type Human = any;
            export type Codec = any;
        }
    }
}

export namespace SpectrePse {
    /** */
    /** Queries */
    /** */
    namespace ContractQuery {
        export interface GenerateOnchainTradingAccount extends DPT.ContractQuery {
            (
                origin: DPT.ContractCallOrigin,
                options: DPT.ContractCallOptions,
            ): DPT.CallReturn<
                ContractExecResult
            >;
        }

        export interface Sign extends DPT.ContractQuery {
            (
                origin: DPT.ContractCallOrigin,
                options: DPT.ContractCallOptions,
                network: SpectrePse.Network | SpectrePse.Network$.Codec,
                message: number[] | string | PT.Vec<PT.U8>,
            ): DPT.CallReturn<
                DPT.Result$.Codec<
                    DPT.Result$.Codec<
                        PT.Vec<PT.U8>,
                        SpectrePse.Error$.Codec
                    >,
                    InkPrimitives.LangError$.Codec
                >
            >;
        }

        export interface RegisterTradingAccountToSpectre extends DPT.ContractQuery {
            (
                origin: DPT.ContractCallOrigin,
                options: DPT.ContractCallOptions,
                message: number[] | string | PT.Vec<PT.U8>,
            ): DPT.CallReturn<
                DPT.Result$.Codec<
                    DPT.Result$.Codec<
                        PT.Vec<PT.U8>,
                        SpectrePse.Error$.Codec
                    >,
                    InkPrimitives.LangError$.Codec
                >
            >;
        }

        export interface GetPublicKeys extends DPT.ContractQuery {
            (
                origin: DPT.ContractCallOrigin,
                options: DPT.ContractCallOptions,
            ): DPT.CallReturn<
                DPT.Result$.Codec<
                    DPT.Result$.Codec<
                        SpectrePse.OnchainTradingPublicKeys$.Codec,
                        SpectrePse.Error$.Codec
                    >,
                    InkPrimitives.LangError$.Codec
                >
            >;
        }
    }

    interface MapMessageQuery extends DPT.MapMessageQuery {
        generateOnchainTradingAccount: ContractQuery.GenerateOnchainTradingAccount;
        sign: ContractQuery.Sign;
        registerTradingAccountToSpectre: ContractQuery.RegisterTradingAccountToSpectre;
        getPublicKeys: ContractQuery.GetPublicKeys;
    }

    /** */
    /** Transactions */
    /** */
    namespace ContractTx {
        export interface GenerateOnchainTradingAccount extends DPT.ContractTx {
            (options: ContractOptions): DPT.SubmittableExtrinsic;
        }
    }

    interface MapMessageTx extends DPT.MapMessageTx {
        generateOnchainTradingAccount: ContractTx.GenerateOnchainTradingAccount;
    }

    /** */
    /** Contract */
    /** */
    export declare class Contract extends DPT.Contract {
        get query(): MapMessageQuery;
        get tx(): MapMessageTx;
    }

    /** */
    /** Contract factory */
    /** */
    export declare class Factory extends DevPhase.ContractFactory<Contract> {
        instantiate(constructor: "seeding", params: [number[] | string | PT.Vec<PT.U8>], options?: DevPhase.InstantiateOptions): Promise<Contract>;
    }
}
