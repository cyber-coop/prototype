-- All the signatures!!

CREATE MATERIALIZED VIEW signatures
AS
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Dogecoin Testnet' as network FROM dogecoin_testnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Dogecoin Mainnet' as network FROM dogecoin_mainnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Namecoin Mainnet' as network FROM namecoin_mainnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Litecoin Mainnet' as network FROM litecoin_mainnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Litecoin Testnet' as network FROM litecoin_testnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Bitcoin Mainnet' as network FROM bitcoin_mainnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, index, (CASE get_byte(sigscript, 4) WHEN 33 THEN substring(sigscript, 7, get_byte(sigscript, 4)-1) ELSE substring(sigscript, 6, get_byte(sigscript, 4)) END) AS r, (CASE get_byte(sigscript, get_byte(sigscript, 4)+6) WHEN 33 THEN substring(sigscript, get_byte(sigscript, 4)+9, get_byte(sigscript, get_byte(sigscript, 4)+6)-1) ELSE substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) END) AS s, 'Bitcoin Testnet' as network FROM bitcoin_testnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602')
UNION ALL
SELECT txid, 0 AS index, r, s, 'Ethereum Rinkeby' as network FROM ethereum_rinkeby.transactions
UNION ALL
SELECT txid, 0 AS index, r, s, 'Ethereum Ropsten' as network FROM ethereum_ropsten.transactions
UNION ALL
SELECT txid, 0 AS index, r, s, 'Ethereum Goerli' as network FROM ethereum_goerli.transactions
UNION ALL
SELECT txid, 0 AS index, r, s, 'Ethereum Sepolia' as network FROM ethereum_sepolia.transactions
UNION ALL
SELECT txid, 0 AS index, r, s, 'Ethereum Mainnet' as network FROM ethereum_mainnet.transactions;