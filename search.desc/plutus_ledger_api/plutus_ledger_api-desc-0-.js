searchState.loadedDescShard("plutus_ledger_api", 0, "Proptest strategies for most common types\nGolden test data\nPlutus Data related types and traits\nPlutus types and utilities for Plutus V1\nPlutus types and utilities for Plutus V2\nPlutus types and utilities for Plutus V3\nCreate an empty container.\nCreate a container C from one element.\nUnion two BTreeMaps, call f to resolve conflicts if …\nConvert a cardano-serialization-lib type to its …\nConvert a cardano-serialization-lib type to its …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nConvert a plutus-ledger-api type to its …\nConvert a plutus-ledger-api type to its …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nProptest strategies for most common types\nProptest strategies for most common primitive types\nProptest strategies for Plutus V1 types\nProptest strategies for Plutus V2 types\nProptest strategies for Plutus V3 types\nStrategy to generate an arbitrary boolean\nStrategy to generate an arbitrary bytestring\nStrategy to generate an arbitrary character\nStrategy to generate a complicated data structure\nStrategy to generate an arbitrary BigInt\nStrategy to generate an arbitrary non-negative BigInt\nStrategy to generate an arbitrary string\nStrategy to generate a Cardano address\nStrategy to generate an asset class\nStrategy to generate an AssocMap, given the strategies to …\nStrategy to generate a certificate index.\nStrategy to generate a chain pointer\nStrategy to generate a credential\nStrategy to generate a currency symbol\nStrategy to generate a DCert\nStrategy to generate a Datum\nStrategy to generate a Datum hash\nStrategy to generate Ed25519 public key hash\nStrategy to generate an Extended set\nStrategy to generate an extended POSIX time\nStrategy to generate a Interval\nStrategy to generate a Interval\nStrategy to generate an arbitrary bytestring with a fixed …\nStrategy to generate a LowerBound\nStrategy to generate a minting policy hash\nStrategy to generate a Value\nStrategy to generate a PaymentPubKeyHash\nStrategy to generate an arbitrary PlutusData with a …\nStrategy to generate a PlutusInterval\nStrategy to generate a PlutusInterval\nStrategy to generate a POSIX Time\nStrategy to generate a Redeemer\nStrategy to generate a Datum hash\nStrategy to generate a ScriptContext\nStrategy to generate a ScriptHash\nStrategy to generate a ScriptPurpose\nStrategy to generate a slot number\nStrategy to generate a staking credential\nStrategy to generate a token name\nStrategy to generate a transaction hash\nStrategy to generate a transaction index\nStrategy to generate a TransactionInfo. Note that its …\nStrategy to generate a transaction input\nStrategy to generate transaction output\nStrategy to generate a TxInInfo\nStrategy to generate an UpperBound\nStrategy to generate a validator hash\nStrategy to generate a Value\nStrategy to generate an output datum\nStrategy to generate transaction output\nStrategy to generate a TxInInfo\nStrategy to generate change parameters\nStrategy to generate cold committee credentials\nStrategy to generate committees\nStrategy to generate constitutions\nStrategy to generate DReps\nStrategy to generate DRep credentials\nStrategy to generate delegatees\nStrategy to generate governance actions\nStrategy to generate governance action ids\nStrategy to generate hot committee credentials\nStrategy to generate protocol procedures\nStrategy to generate protocol versions\nStrategy to generate rationals\nStrategy to generate script contexts\nStrategy to generate script info\nStrategy to generate script purposes\nStrategy to generate a transaction hash\nStrategy to generate transaction info\nStrategy to generate a transaction input\nStrategy to generate tx certs\nStrategy to generate a TxInInfo\nStrategy to generate votes\nStrategy to generate voters\nGolden test data or Plutus V1 types\nGolden test data or Plutus V2 types\nGolden test data or Plutus V3 types (incomplete)\nData representation of on-chain data such as Datums and …\nDeserialise a Plutus data using parsers for each variant\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGiven a PlutusData, parse it as PlutusData::Constr and its …\nGiven a PlutusData, parse it as PlutusData::Constr and …\nGiven a vector of PlutusData, parse it as an array whose …\nGiven a PlutusData, parse it as PlutusData::List. Return …\nTypes related to Cardano addresses\nTypes for cryptographic primitives, and other lower level …\nTypes related to Plutus Datums\nTypes related to PlutusInterval\nTypes related to Plutus Redeemers\nTypes related to Plutus scripts\nTypes related to Cardano transactions.\nTypes related to Cardano values, such as Ada and native …\nShelley Address for wallets or validators\nAddress with network information. The <code>WithExtraInfo</code> …\nPosition of the certificate in a certain transaction\nIn an address, a chain pointer refers to a point of the …\nA public key hash or validator hash credential (used as a …\nNumber of slots elapsed since genesis\nCredential (public key hash or pointer) used for staking\nPosition of a transaction in a given slot This is not …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nInserts a key-value pair into the map.\nCalls <code>U::from(self)</code>.\nRemoves a key from the map, returning the value at the key …\nED25519 public key hash This is the standard cryptography …\nA bytestring in the Cardano ledger context\nStandard public key hash used to verify a transaction …\nStandard public key hash used to verify a staking\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPiece of information associated with a UTxO encoded into a …\nblake2b-256 hash of a datum\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA set extended with a positive and negative infinity.\nAn abstraction over <code>PlutusInterval</code>, allowing valid values …\nAn interval of <code>T</code>s.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPiece of information attached to a transaction when …\nblake2b-256 hash of a datum\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nHash of a minting policy script\nHash of a Plutus script\nIdentifier of a validator script\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPartial representation of digests of certificates on the …\nPOSIX time is measured as the number of milliseconds since …\nA digest of the PoolParam\nThe context that is presented to the currently-executing …\nThe purpose of the script that’s currently running.\n32-bytes blake2b256 hash of a transaction body.\nA pending transaction as seen by validator scripts, also …\nAn input of a transaction\nAn output of a transaction\nAn input of a pending transaction.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAssetClass is uniquely identifying a specific asset\nIdentifier of a currency, which could be either Ada (or …\nName of a token. This can be any arbitrary bytearray\nA value that can contain multiple asset classes\nAda tokenname (empty bytestring)\nCreate a Value containing only ada tokens, given the …\nApply a predicate to tokens.\nApply a function to each token of the value. If the result …\nCreate a vector with each distinct value Warning: is the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert a UTF8 string into a TokenName (use from_str to …\nLookup the quantity of ada(unit: lovelace).\nLookup the quantity of the given token.\nInsert a new token into the value, or replace the existing …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn true if the value don’t have any entries.\nApply a function to each token of the value, and use its …\nRemove all tokens whose quantity is zero.\nCreate a Value containing only the given quantity of the …\nConvert TokenName to string if it is a valid UTF8 …\nTypes related to Plutus Datums\nTypes related to Cardano transactions.\nOptional datum of a transaction\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe context that is presented to the currently-executing …\nA pending transaction as seen by validator scripts, also …\nAn output of a transaction\nAn input of a pending transaction.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTypes related to rational values\nTypes related to Cardano transactions.\nRepresents an arbitrary-precision ratio.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nAuthorize a Hot credential for a specific Committee member…\nA Plutus Data object containing proposed parameter changes.\nDelegate staking credential to a Delegatee\nSimilar to TransactionInput, but for GovernanceAction.\nPropose to update protocol version\nPropose to modify the constitution or its guardrail script\nPropose to create a state of no-confidence in the current …\nPropose to change the protocol parameters\nA digest of the PoolParams\nThe retirement certificate and the Epoch in which the …\nRegister a DRep with a deposit value. The optional anchor …\nRegister and delegate staking credential to a Delegatee in …\nRegister staking credential with an optional deposit amount\n32-bytes blake2b256 hash of a transaction body.\nAn input of a transaction\nPropose to withdraw from the cardano treasury\nAn input of a pending transaction.\nUnRegister a DRep with mandatory refund value\nUn-Register staking credential with an optional refund …\nPropose to update the members of the constitutional …\nUpdate a DRep. The optional anchor is omitted.\nOptional guardrail script\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCommittee members with epoch number when each of them …\nQuorum of the committee that is necessary for a successful …")