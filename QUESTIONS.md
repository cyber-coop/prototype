# Questions

## Combien de Bitcoins ont été minés au total ?

Pour cela on additione le total de tout les utxos.

## Combien de Bitcoins ont été perdus ou sont considérés comme inactifs ?

Il faut d'abord décider à partir de quand on considère qu'un utxos est "inactif".

## C'est quoi la répartition des wallets en fonction du montant de Bitcoins détenus ?

C'est impossible à savoir car plusieurs addresses peuvent être posséder par le même wallet. La pratique veut qu'on créait une nouvelle clé public pour chaque transaction.

Par contre c'est possible pour Ethereum ou toute les blockchains "account-based" car une address = un account.

## Combien de wallets sont actifs et comment tu as définis un wallet actif ?

Pareil il est difficile de savoir les addresses qui appartienent à un même wallet.

## Quel est le volume moyen de transactions quotidiennes sur Bitcoin ?

Bonne question!

## Quelle est l'évolution du nombre de tx/s sur Bitcoin au fil des ans ?

Bonne question!

## Quelle est le hashrate moyen du réseau Bitcoin depuis sa création ?

Bonne question!

## Combien y a t'il de colored bitcoin ?

... Comment savoir quels transactions est un colored coin.

## Quel est le pourcentage d'utilisateurs sur Bitcoin qui effectuent des tx régulièrement par rapport aux holders ?

Impossible à dire.

## Quels sont les principaux types de tx effectuées en Bitcoin (achats, transferts entre wallets, paiements de factures, etc.) ?

Impossible à dire.

## Quel est le montant moyen d'une tx en Bitcoin ? Comment cette moyenne a-t-elle évolué au fil des ans ?

Bonne question!

## Quelle est la part des tx effectuées en utilisant des solutions de confidentialité comme les mixers de bitcoins ou les transactions CoinJoin ?

Normalement on ne doit pas pouvoir savoir ce qui relève de CoinJoin ou pas... Et CoinShuffle est un meilleur mixer!!

## Est il possible de detecter les Wallet multisigs utilisés pour le LN ? Et si oui combien en a t'il ?

Oui c'est possible!

## Quelle est la part des tx en Bitcoin utilisant des adresses SegWit ou Native SegWit (Bech32) ?

Je ne suis pas sure que ce soit refléter sur les txs...

## Comment identifier des wallets appartenant potentiellement à une même personne ?

One ne peut pas!

## Le nombre de noeud BTC ? En France ?

C'est possible!!!! Mais on va utiliser les messages P2P pour ça.


-----------


## Total clé public unique

### Dogecoin
```
blockchains=# SELECT COUNT(DISTINCT pubkey) FROM dogecoin_mainnet.pubkeys;
 count  
--------
 519125
(1 row)
```

### Namecoin

```
blockchains=# SELECT COUNT(DISTINCT pubkey) FROM namecoin_mainnet.pubkeys;
  count  
---------
 3221352
(1 row)
```

## Total hash de clé public

### Dogecoin


## Hash de clé public bizarre

### Bitcoin

```
blockchains=# SELECT * FROM bitcoin_mainnet.pubkeys_hash LIMIT 100;
                 pubkeyhash                 
--------------------------------------------
 \x0000000000000000000000000000000000000000
 \x0000000000000000000000000000000000000001
 \x0000000000000000000000000000000000000002
 \x0000000000000000000000000000000000000003
 \x0000000000000000000000000000000000000004
 \x0000000000000000000000000000000000000005
 \x0000000000000000000000000000000000000006
 \x0000000000000000000000000000000000000007
 \x0000000000000000000000000000000000000008
 \x000000000000000000000000000000000000000a
 \x0000000000000000000000000000000000000011
 \x000000000000000000000000000000000000001a
 ...
```

### Dogecoin

```
blockchains=# SELECT * FROM dogecoin_mainnet.pubkeys_hash LIMIT 10;
                 pubkeyhash                 
--------------------------------------------
 \x0000000000000000000000000000000000000000
 \x0000000000000000000000000000000000000001
 \x0000000000000000000000000000000000000002
 \x0000000000000000000000000000000000000003
 \x00000000000000000000000000000000000000f6
 \x000000000000000000000000000000000058595a
 \x0000000000000000000000000000000000746578
 \x0000000000000000000000000000000100000000
 \x0000000000000000000000000000000258595a20
 \x0000000000000000000000000000008613000013
(10 rows)
```

## Clé publique bizarre

### Bitcoin

```
blockchains=# SELECT * FROM bitcoin_mainnet.pubkeys LIMIT 10;
                                                                pubkey                                                                
--------------------------------------------------------------------------------------------------------------------------------------
 \x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
 \x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
 \x004ae7075c54fba20a9a53bb97730f3e618a24d0bb90bc6836b6cd26a5497718384548258ec94fc2cda2b6d8ae0051ef8fa3ae190b2b679e19f2f28327addae8e6
 \x006400004844c40bf573e8f3bf9aa2a477be82950c2ffd1ac3814deb530f424ecd238943cfd19a524fb315cf6908e055c10fa39d457ddf01225c44c0e1f8c6d836
 \x00c0a5f1fa22be56628eb07ac1ddfc08486183faceca07f48115498186c9a18e7c2e86eea609031398434d5dabd79eab9a0a568bf7b99b2e51fd4388b6e4515293
 \x02171fe727f27bebfe68bcdf28fa20192938caba3f030cd835e4be6467e1aed2cefcf2fb7eee2ba9ffff05637003eceb613d10358e20a0c8be61538d504ed0f0a5
 \x02853426416d5bc5492d94ec805689f1179147d6855a3da4b43672dd7b94622e7405aadcdfd7f73f8def7144d9b4f74b6aafee98fe1b1ae17406a9edc7de73986d
 \x0400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
 \x0400000000000000000000000000000061613465393834633536353561363737373136353339616363386362633063653239333331343239000000000000000000
 \x0400000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
(10 rows)
```

## Total de Dogecoin en circulation


