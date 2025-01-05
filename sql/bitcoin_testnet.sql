ANALYZE bitcoin_testnet.blocks;
ANALYZE bitcoin_testnet.transactions;

-- Bitcoin testnet

--- Create Primary key and Foreign key
ALTER TABLE bitcoin_testnet.blocks ADD CONSTRAINT hash_pk PRIMARY KEY (hash);

ALTER TABLE bitcoin_testnet.transactions ADD CONSTRAINT txid_pk PRIMARY KEY (txid, block);
ALTER TABLE bitcoin_testnet.transactions ADD CONSTRAINT block_fk FOREIGN KEY(block) REFERENCES bitcoin_testnet.blocks(hash);

ALTER TABLE bitcoin_testnet.txins ADD CONSTRAINT txins_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE bitcoin_testnet.txins ADD CONSTRAINT transaction_fk FOREIGN KEY(txid) REFERENCES bitcoin_testnet.transactions(txid);

ALTER TABLE bitcoin_testnet.txouts ADD CONSTRAINT txouts_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE bitcoin_testnet.txouts ADD CONSTRAINT transactions_fk FOREIGN KEY(txid) REFERENCES bitcoin_testnet.transactions(txid);

--- Create Index
---- We use B-Tree indexing because later we want to do more insert of new transactions and it is supposedly faster 

CREATE INDEX i_blocknumber ON bitcoin_testnet.blocks using btree (height);
CREATE INDEX i_txid ON bitcoin_testnet.transactions using btree (txid);
CREATE INDEX i_block ON bitcoin_testnet.transactions using btree (block);
CREATE INDEX i_txin ON bitcoin_testnet.txins using btree (txid);
CREATE INDEX i_txout ON bitcoin_testnet.txouts using btree (txid);
CREATE INDEX i_outputhash ON bitcoin_testnet.txins using btree (txid);