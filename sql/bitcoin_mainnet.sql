-- Remove duplicates
-- Inpsired by (https://postgres.cz/wiki/PostgreSQL_SQL_Tricks#Delete_duplicate_rows_with_window_analytic_functions)

-- DELETE FROM bitcoin_mainnet.transactions WHERE ctid = ANY(SELECT ctid FROM (SELECT row_number() OVER (PARTITION BY txid), ctid FROM bitcoin_mainnet.transactions) x WHERE x.row_number > 1);

-- DELETE FROM bitcoin_mainnet.txins WHERE ctid = ANY(SELECT ctid FROM (SELECT row_number() OVER (PARTITION BY txid, index), ctid FROM bitcoin_mainnet.txins) x WHERE x.row_number > 1);

-- DELETE FROM bitcoin_mainnet.txouts WHERE ctid = ANY(SELECT ctid FROM (SELECT row_number() OVER (PARTITION BY txid, index), ctid FROM bitcoin_mainnet.txouts) x WHERE x.row_number > 1);

ANALYZE bitcoin_mainnet.blocks;
ANALYZE bitcoin_mainnet.transactions;

-- Bitcoin mainnet

--- Create Primary key and Foreign key
ALTER TABLE bitcoin_mainnet.blocks ADD CONSTRAINT hash_pk PRIMARY KEY (hash);

ALTER TABLE bitcoin_mainnet.transactions ADD CONSTRAINT txid_pk PRIMARY KEY (txid, block);
ALTER TABLE bitcoin_mainnet.transactions ADD CONSTRAINT block_fk FOREIGN KEY(block) REFERENCES bitcoin_mainnet.blocks(hash);

ALTER TABLE bitcoin_mainnet.txins ADD CONSTRAINT txins_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE bitcoin_mainnet.txins ADD CONSTRAINT transaction_fk FOREIGN KEY(txid) REFERENCES bitcoin_mainnet.transactions(txid);

ALTER TABLE bitcoin_mainnet.txouts ADD CONSTRAINT txouts_pk PRIMARY KEY (txid,index);
-- Our problem is that txid are not unique... coinbase txid could be exactly the same as an other creating the same txid
-- ALTER TABLE bitcoin_mainnet.txouts ADD CONSTRAINT transactions_fk FOREIGN KEY(txid) REFERENCES bitcoin_mainnet.transactions(txid);

--- Create Index
---- We use B-Tree indexing because later we want to do more insert of new transactions and it is supposedly faster 

CREATE INDEX i_blocknumber ON bitcoin_mainnet.blocks using btree (height);
CREATE INDEX i_txid ON bitcoin_mainnet.transactions using btree (txid);
CREATE INDEX i_block ON bitcoin_mainnet.transactions using btree (block);
CREATE INDEX i_txin ON bitcoin_mainnet.txins using btree (txid);
CREATE INDEX i_txout ON bitcoin_mainnet.txouts using btree (txid);
CREATE INDEX i_outputhash ON bitcoin_mainnet.txins using btree (txid);