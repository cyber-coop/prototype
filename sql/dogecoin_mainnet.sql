ANALYZE dogecoin_mainnet.blocks;
ANALYZE dogecoin_mainnet.transactions;

-- Dogecoin mainnet

--- Create Primary key and Foreign key
ALTER TABLE dogecoin_mainnet.blocks ADD CONSTRAINT hash_pk PRIMARY KEY (hash);

ALTER TABLE dogecoin_mainnet.transactions ADD CONSTRAINT txid_pk PRIMARY KEY (txid, block);
ALTER TABLE dogecoin_mainnet.transactions ADD CONSTRAINT block_fk FOREIGN KEY(block) REFERENCES dogecoin_mainnet.blocks(hash);

ALTER TABLE dogecoin_mainnet.txins ADD CONSTRAINT txins_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE dogecoin_mainnet.txins ADD CONSTRAINT transaction_fk FOREIGN KEY(txid) REFERENCES dogecoin_mainnet.transactions(txid);

ALTER TABLE dogecoin_mainnet.txouts ADD CONSTRAINT txouts_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE dogecoin_mainnet.txouts ADD CONSTRAINT transactions_fk FOREIGN KEY(txid) REFERENCES dogecoin_mainnet.transactions(txid);

--- Create Index
---- We use B-Tree indexing because later we want to do more insert of new transactions and it is supposedly faster 

CREATE INDEX i_blocknumber ON dogecoin_mainnet.blocks using btree (height);
CREATE INDEX i_txid ON dogecoin_mainnet.transactions using btree (txid);
CREATE INDEX i_block ON dogecoin_mainnet.transactions using btree (block);
CREATE INDEX i_txin ON dogecoin_mainnet.txins using btree (txid);
CREATE INDEX i_txout ON dogecoin_mainnet.txouts using btree (txid);
CREATE INDEX i_outputhash ON dogecoin_mainnet.txins using btree (txid);