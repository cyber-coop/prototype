ANALYZE litecoin_mainnet.blocks;
ANALYZE litecoin_mainnet.transactions;

-- Litecoin mainnet

--- Create Primary key and Foreign key
ALTER TABLE litecoin_mainnet.blocks ADD CONSTRAINT hash_pk PRIMARY KEY (hash);

ALTER TABLE litecoin_mainnet.transactions ADD CONSTRAINT txid_pk PRIMARY KEY (txid, block);
ALTER TABLE litecoin_mainnet.transactions ADD CONSTRAINT block_fk FOREIGN KEY(block) REFERENCES litecoin_mainnet.blocks(hash);

ALTER TABLE litecoin_mainnet.txins ADD CONSTRAINT txins_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE litecoin_mainnet.txins ADD CONSTRAINT transaction_fk FOREIGN KEY(txid) REFERENCES litecoin_mainnet.transactions(txid);

ALTER TABLE litecoin_mainnet.txouts ADD CONSTRAINT txouts_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE litecoin_mainnet.txouts ADD CONSTRAINT transactions_fk FOREIGN KEY(txid) REFERENCES litecoin_mainnet.transactions(txid);

--- Create Index
---- We use B-Tree indexing because later we want to do more insert of new transactions and it is supposedly faster 

CREATE INDEX i_blocknumber ON litecoin_mainnet.blocks using btree (height);
CREATE INDEX i_txid ON litecoin_mainnet.transactions using btree (txid);
CREATE INDEX i_block ON litecoin_mainnet.transactions using btree (block);
CREATE INDEX i_txin ON litecoin_mainnet.txins using btree (txid);
CREATE INDEX i_txout ON litecoin_mainnet.txouts using btree (txid);
CREATE INDEX i_outputhash ON litecoin_mainnet.txins using btree (txid);