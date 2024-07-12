# Lola's notes

## Find duplicates

Find duplicates txid in `txs`

```
SELECT txid, COUNT(*) FROM txs GROUP BY txid HAVING COUNT(*) > 1;
```

## Process data

### Find signatures

Signatures are in the scriptsig column.
Query
```
SELECT count(*) FROM txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602');
```

Parse signatures!
```
SELECT substring(sigscript, 6, get_byte(sigscript, 4)) AS r, get_byte(sigscript, get_byte(sigscript, 4)+6), substring(sigscript, get_byte(sigscript, 4)+8, get_byte(sigscript, get_byte(sigscript, 4)+6)) AS s FROM dogecoin_testnet.txins WHERE (substring(sigscript, 1, 4) = '\x48304502') OR (substring(sigscript, 1, 4) = '\x47304402') OR (substring(sigscript, 1, 4) = '\x49304602');
```

UNION VIEW OF ALL SIGNATURES!!
```
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
SELECT txid, 0 AS index, substring(r, 2) AS r, substring(s, 2) AS s, 'Ethereum Rinkeby' as network FROM ethereum_rinkeby.transactions
UNION ALL
SELECT txid, 0 AS index, substring(r, 2) AS r, substring(s, 2) AS s, 'Ethereum Mainnet' as network FROM ethereum_mainnet.transactions;
```

Then create indexes
```
CREATE INDEX index_r ON signatures USING hash (r);
```


WARNING: we have false positive signatures
see
```
 l  |                                   sigscript                                    
----+--------------------------------------------------------------------------------
 38 | \x03304402062f503253482f03fc74150808000001080000000d2f436f696e69756d536572762f
 13 | \x033045020101062f503253482f
 13 | \x033046020101062f503253482f
(3 rows)
```

Find duplicates!
```
CREATE MATERIALIZED VIEW duplicates AS SELECT s.* FROM signatures s JOIN (SELECT r, count(*) FROM signatures GROUP BY r HAVING COUNT(*) > 1) d ON s.r = d.r ORDER BY s.r;
```

### FIND BLOCK MINE WITH MY MINER SOFTWARE
```
SELECT COUNT(*) FROM dogecoin_testnet.txins WHERE encode(sigscript, 'hex') LIKE '%4c6f6c6120697320746865206265737421%';
 count 
-------
 10674
(1 row)
```

### FIND ALL PUBKEYS AND PUBKEYS HASH

#### Find all the pubkeys from P2PK

'ac' is opcode CHECKSIG and it is 172 in decimal. We verify if the script end with CHECKSIG opcode and assume it is P2PK.
key length can be 65 bytes (uncompressed) or 33 (compressed)

NOTES: Don't forget data start with data length see `4104ffd03de44a6e11b9917f3a29f9443283d9871c9d743ef30d5eddcd37094b64d1b3d8090496b53256786bf5c82932ec23c3b74d9f05a6f9
5a8b5529352656664bac`

```
CREATE MATERIALIZED VIEW dogecoin_testnet.pubkeys
AS
SELECT substr(pkscript,2,65) as pubkey FROM dogecoin_testnet.txouts WHERE length(pkscript) = 67 AND get_byte(pkscript, 66) = 172
UNION ALL
SELECT substr(pkscript,2,33) as pubkey FROM dogecoin_testnet.txouts WHERE length(pkscript) = 35 AND get_byte(pkscript, 34) = 172;
```

How many unique pubkeys ?
```
SELECT COUNT(DISTINCT pubkey) FROM dogecoin_testnet.pubkeys;
```

#### Find all the pubkeyhash from P2PKH

Example of P2SH : `76a9140817fa995b26604c5ed08c160f0bc2141567ce7288ac`

```
SELECT substr(pkscript, 4, 20) as pubkeyhash FROM dogecoin_testnet.txouts WHERE substr(pkscript, 1, 2) = '\x76a9';
```

### UNSPENT UTXOS

We want to get all the unspent utxos so we can determine how many coins have been lost.
```
CREATE MATERIALIZED VIEW dogecoin_testnet.utxos
AS
SELECT a.txid, a.index, a.value, a.pkscript FROM dogecoin_testnet.txouts a
JOIN (SELECT txid, index FROM dogecoin_testnet.txouts
EXCEPT
SELECT outputhash, outputindex FROM dogecoin_testnet.txins) b ON (a.txid = b.txid AND a.index = b.index);
```

Look at utxos blocks height
```
SELECT height, hash, a.txid, a.index, a.value, a.pkscript FROM dogecoin_testnet.utxos a 
JOIN dogecoin_testnet.transactions b ON a.txid = b.txid
JOIN dogecoin_testnet.blocks c ON b.block = c.hash;
```

### Calculate all the distributed coins

The sum of all the values of the unspent transactions tells us how many coins are in circulation.
```
SELECT SUM(value) FROM dogecoin_testnet.utxos;
```

Nombre de Bitcoin non-miné : 3297.705 à hauteur 692500.

### All the transactions that are not P2PK or P2PKH

```
SELECT * FROM dogecoin_testnet.txouts WHERE LENGTH(pkscript) > 0 AND NOT get_byte(pkscript, 0) = 106 AND NOT get_byte(pkscript, 0) = 118 AND NOT get_byte(pkscript, 0) = 65 AND NOT get_byte(pkscript, 0) = 33 AND NOT get_byte(pkscript, 0) = 169;
```

### Get all the sighash single signature

```
SELECT txin.txid, txin.index AS txin_index, txin.sigscript, txout.index AS txout_index FROM dogecoin_testnet.txins txin JOIN dogecoin_testnet.txouts txout ON txin.txid = txout.txid WHERE ((substring(txin.sigscript, 1, 4) = '\x48304502' AND get_byte(txin.sigscript, 72) = 3) OR (substring(txin.sigscript, 1, 4) = '\x47304402' AND get_byte(txin.sigscript, 71) = 3) OR (substring(txin.sigscript, 1, 4) = '\x49304602' AND get_byte(txin.sigscript, 73) = 3)) AND txin.index > txout.index
UNION
SELECT txin.txid, txin.index AS txin_index, txin.sigscript, txout.index AS txout_index FROM dogecoin_mainnet.txins txin JOIN dogecoin_mainnet.txouts txout ON txin.txid = txout.txid WHERE ((substring(txin.sigscript, 1, 4) = '\x48304502' AND get_byte(txin.sigscript, 72) = 3) OR (substring(txin.sigscript, 1, 4) = '\x47304402' AND get_byte(txin.sigscript, 71) = 3) OR (substring(txin.sigscript, 1, 4) = '\x49304602' AND get_byte(txin.sigscript, 73) = 3)) AND txin.index > txout.index;
```

### FIND NAMECOIN DATA

```
SELECT * FROM namecoin_mainnet.txouts WHERE get_byte(pkscript, 0) = 82 OR get_byte(pkscript, 0) = 83;
```

## Indexing data

### Batch blocks

We are reciving by batch of 500 blocks and they are put in the thread queue before being process. I attempted to empty the queue before processing the data hoping speeding up the postgres process. It didn't really helped and instead I have found that there is a hard limit of 1GB for `COPY`.

See - https://github.com/sfackler/rust-postgres/issues/986

### Creating different clients and not calling `finish`

~~I have attempted creating 4 connections for the 4 writers (blocks, transactions, txins and txouts) and avoiding calling `finish` but it doesn't show meaningfull result in term of speed. I am also loosing the ability to see data in database will syncing.~~

I was wrong there is signifiant speed in the long term. So we are not calling `finish` once we are synced.

### SSD

The best way to optimize the indexing would be to have SSD storage instead of HDD. Not tested yet.

### using DEFERRED

We can't DEFERRED primary key that we are using as foreign key. Using DEFERRED will force the check for constraints to happen after COMMIT is being called. It will happen once we are fully synced.

Does it really save time ?

NOT REALLY

### Remove constraints

Just remove the constraints !!!! Way faster that.
When adding constraints you can run : 
```
$ psql -U postgres -d blockchains -a -f /constraints.sql
```

Check if we have constraints 
```
$ \d+ bitcoin_mainnet.blocks
```

### Run docker detach execution

```
docker exec -d devops-postgres-1 psql -U postgres -d blockchains -c "CREATE MATERIALIZED VIEW duplicates AS SELECT s.* FROM signatures s JOIN (SELECT r, count(*) FROM signatures GROUP BY r HAVING COUNT(*) > 1) d ON s.r = d.r ORDER BY s.r;"
```

Run constraints
```
$ docker exec -d contrib-postgres-1 psql -U postgres -d blockchains -L /bitcoin_mainnet.log -a -f /constraints/bitcoin_mainnet.sql
```
check what is running inside the postgres container after
```
$ for prc in /proc/*/cmdline; { (printf "$prc "; cat -A "$prc") | sed 's/\^@/ /g;s|/proc/||;s|/cmdline||'; echo; }
```

TODO: the logs should be available in a `log` folder.

### See all Materialized View

```
SELECT oid::regclass::text
FROM   pg_class
WHERE  relkind = 'm';
```

### Current duplicates

8540