/* coinentry.rs - coin entry object for hsd
 * Copyright (c) 2019-2020, Urkel Labs (MIT License).
 * https://github.com/HandshakeAlliance/rsd
 */

/// Need to get Extended Primitives included in this file found in Urkel Repo
// const bio = require('bufio');
// const {encoding} = bio;

use handshake_primitives::Coin; /// This needs to be written 
use handshake_primitives::Output;
crate::compress::*;

/// Need to set these
/* Constants
 * const NUM_FLAGS = 1;
 * const MAX_HEIGHT = ((1 << (32 - NUM_FLAGS)) >>> 0) - 1;
 */ 

/* Coin Entry Types
 * Info --> represents an unspent output 
 * Version {Number} - transaction version
 * Height {Number} - transaction height (unconfirmed = -1)
 * Coinbase {Boolean} - whether the transaction is a coinbase   
 * Output {Output}
 * Spent {Boolean}
 * Raw {Buffer}
 */
 
pub struct CoinEntry {
    pub version,
    pub height,
    pub coinbase: bool,
    pub output,
    pub spent: bool,
    pub raw,
}

impl CoinEntry {

    // Convert coin entry to an output
    pub fn to_output() {

    }

    // Convert coin entry to a coin
    pub fn to_coin() {

    }

    // Inject properties from TX
    pub fn from_output() {
    
    }

}










// ################################# BEGIN HSD CODE ######################################


//class CoinEntry extends bio.Struct {
//  /**
//   * Create a coin entry.
//   * @constructor
//   */
//
//  constructor() {
//    super();
//    this.version = 1;
//    this.height = -1;
//    this.coinbase = false;
//    this.output = new Output();
//    this.spent = false;
//    this.raw = null;
//  }
//
//  /**
//   * Convert coin entry to an output.
//   * @returns {Output}
//   */
//
//  toOutput() {
//    return this.output;
//  }
//
//  /**
//   * Convert coin entry to a coin.
//   * @param {Outpoint} prevout
//   * @returns {Coin}
//   */
//
//  toCoin(prevout) {
//    const coin = new Coin();
//    coin.version = this.version;
//    coin.height = this.height;
//    coin.coinbase = this.coinbase;
//    coin.value = this.output.value;
//    coin.address = this.output.address;
//    coin.covenant = this.output.covenant;
//    coin.hash = prevout.hash;
//    coin.index = prevout.index;
//    return coin;
//  }
//
//  /**
//   * Inject properties from TX.
//   * @param {TX} tx
//   * @param {Number} index
//   */
//
//  fromOutput(output) {
//    this.output = output;
//    return this;
//  }
//
//  /**
//   * Instantiate a coin from a TX
//   * @param {TX} tx
//   * @param {Number} index - Output index.
//   * @returns {CoinEntry}
//   */
//
//  static fromOutput(output) {
//    return new this().fromOutput(output);
//  }
//
//  /**
//   * Inject properties from TX.
//   * @param {TX} tx
//   * @param {Number} index
//   */
//
//  fromCoin(coin) {
//    this.version = coin.version;
//    this.height = coin.height;
//    this.coinbase = coin.coinbase;
//    this.output.value = coin.value;
//    this.output.address = coin.address;
//    this.output.covenant = coin.covenant;
//    return this;
//  }
//
//  /**
//   * Instantiate a coin from a TX
//   * @param {TX} tx
//   * @returns {CoinEntry}
//   */
//
//  static fromCoin(coin) {
//    return new this().fromCoin(coin);
//  }
//
//  /**
//   * Inject properties from TX.
//   * @param {TX} tx
//   * @param {Number} index
//   */
//
//  fromTX(tx, index, height) {
//    assert(typeof index === 'number');
//    assert(typeof height === 'number');
//    assert(index >= 0 && index < tx.outputs.length);
//    this.version = tx.version;
//    this.height = height;
//    this.coinbase = tx.isCoinbase();
//    this.output = tx.outputs[index];
//    return this;
//  }
//
//  /**
//   * Instantiate a coin from a TX
//   * @param {TX} tx
//   * @param {Number} index - Output index.
//   * @returns {CoinEntry}
//   */
//
//  static fromTX(tx, index, height) {
//    return new this().fromTX(tx, index, height);
//  }
//
//  /**
//   * Calculate size of coin.
//   * @returns {Number}
//   */
//
//  getSize() {
//    if (this.raw)
//      return this.raw.length;
//
//    let size = 0;
//    /// This is a custom method 
//    size += encoding.sizeVarint(this.version);
//    size += 4;
//    size += compress.size(this.output);
//
//    return size;
//  }
//
//  /**
//   * Write the coin to a buffer writer.
//   * @param {BufferWriter} bw
//   */
//
//  write(bw) {
//    if (this.raw) {
//      bw.writeBytes(this.raw);
//      return bw;
//    }
//
//    let height = this.height;
//    let field = 0;
//
//    if (this.coinbase)
//      field |= 1;
//
//    if (height === -1)
//      height = MAX_HEIGHT;
//
//    field |= height << NUM_FLAGS;
//
//    bw.writeVarint(this.version);
//    bw.writeU32(field);
//    compress.pack(this.output, bw);
//
//    return bw;
//  }
//
//  /**
//   * Serialize the coin.
//   * @returns {Buffer}
//   */
//
//  encode() {
//    if (!this.raw)
//      this.raw = super.encode();
//
//    return this.raw;
//  }
//
//  /**
//   * Inject properties from serialized buffer writer.
//   * @private
//   * @param {BufferReader} br
//   */
//
//  read(br) {
//    const version = br.readVarint();
//    const field = br.readU32();
//
//    let height = field >>> NUM_FLAGS;
//
//    if (height === MAX_HEIGHT)
//      height = -1;
//
//    this.version = version;
//    this.coinbase = (field & 1) !== 0;
//    this.height = height;
//
//    compress.unpack(this.output, br);
//
//    return this;
//  }
//
//  /**
//   * Inject properties from serialized data.
//   * @private
//   * @param {Buffer} data
//   */
//
//  decode(data) {
//    super.decode(data);
//    this.raw = data;
//    return this;
//  }
//}
//
///*
// * Expose
// */
//
//module.exports = CoinEntry;
