import struct
import hashlib
import base58

TRANSACTION_VERSION = 1
TRANSACTION_FLAG_ALL = 1
TRANSACTION_FLAG_SINGLE = 3

def hash(data):
    sha = hashlib.sha256()
    sha.update(data)
    return sha.digest()

def hash160(hex_str):
    sha = hashlib.sha256()
    rip = hashlib.new('ripemd160')
    sha.update(hex_str)
    rip.update( sha.digest() )
    return rip.hexdigest()  # .hexdigest() is hex ASCII


def serialize_tx(txid, index, network_name, cursor, flag=TRANSACTION_FLAG_ALL):

    network = network_name.lower().replace(" ", "_")

    query = (
        "SELECT * FROM " + network + ".transactions WHERE txid = '\\x"
        + txid
        + "';"
    )
    cursor.execute(query)
    tx = cursor.fetchone()

    # Recreate transaction
    tx_bytes = struct.pack("i", tx[0])
    query = "SELECT * FROM " + network + ".txins WHERE txid = '\\x" + txid + "';"
    cursor.execute(query)
    txins = cursor.fetchall()
    # sorting by index the result of the query. really important.
    txins.sort()

    # wont work if size bigger than 253
    assert len(txins) < 253
    tx_bytes += struct.pack("B", len(txins))
    for i, txin in enumerate(txins):
        tx_bytes += struct.pack("32s", bytes(txin[2]))
        tx_bytes += struct.pack("I", txin[3])
        if txin[1] == index:
            # Need to look for old output!
            query = (
                "SELECT * FROM " + network + ".txouts WHERE txid = '\\x"
                + txin[2].hex()
                + "' AND index = "
                + str(txin[3])
                + ";"
            )
            cursor.execute(query)
            previous = cursor.fetchone()
            # wont work if size bigger than 253
            assert len(previous[3]) < 253
            l = len(previous[3])
            tx_bytes += struct.pack("B", l)
            tx_bytes += struct.pack(str(l) + "s", bytes(previous[3]))
        else:
            # No output script
            tx_bytes += struct.pack("B", 0)
        tx_bytes += struct.pack("I", txin[5])

    query = (
        "SELECT * FROM " + network + ".txouts WHERE txid = '\\x" + txid + "';"
    )
    cursor.execute(query)
    txouts = cursor.fetchall()

    # https://bitcoin.stackexchange.com/questions/114850/sighash-single-with-no-corresponding-output
    # https://medium.com/@bitaps.com/exploring-bitcoin-signature-hash-types-15427766f0a9
    if flag == TRANSACTION_FLAG_SINGLE and index > len(txouts):
        return bytes.fromhex("0100000000000000000000000000000000000000000000000000000000000000")

    # wont work if size bigger than 253
    assert len(txouts) < 253
    tx_bytes += struct.pack("B", len(txouts))

    for txout in txouts:
        tx_bytes += struct.pack("L", txout[2])
        pkscript = bytes(txout[3])
        tx_bytes += struct.pack("B", len(pkscript))
        tx_bytes += struct.pack(str(len(pkscript)) + "s", pkscript)

    tx_bytes += struct.pack("I", tx[3])
    # Signing flag
    tx_bytes += struct.pack("i", flag)

    return tx_bytes



def to_address(pubkey, network):
    key_hash = network + hash160(bytearray.fromhex(pubkey))

    # Obtain signature:

    sha = hashlib.sha256()
    sha.update( bytearray.fromhex(key_hash) )
    checksum = sha.digest()
    sha = hashlib.sha256()
    sha.update(checksum)
    checksum = sha.hexdigest()[0:8]

    return (base58.b58encode( bytes(bytearray.fromhex(key_hash + checksum)) )).decode('utf-8')