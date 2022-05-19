1:
```shell
./target/release/node-template key generate --scheme Sr25519 --password-interactive
```

Secret phrase:       circle despair shy flat artist crane mouse comfort thrive chimney near cradle
Network ID:        substrate
Secret seed:       0xd7eb03f23b26b2f09cc91b4b1433f4902536afbab63be6df418b6f816a4a2009
Public key (hex):  0xdc20b037c7d48eb4dfb5d3ea742771b0a1f383426a3fefe50f8b30656677823a
Account ID:        0xdc20b037c7d48eb4dfb5d3ea742771b0a1f383426a3fefe50f8b30656677823a
Public key (SS58): 5H3L6tGb37PEo23FLYny53UoaVNgdw4rNPKSSsL7C948h5d3
SS58 Address:      5H3L6tGb37PEo23FLYny53UoaVNgdw4rNPKSSsL7C948h5d3

2:
```shell
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "circle despair shy flat artist crane mouse comfort thrive chimney near cradle"
```

Secret phrase:       circle despair shy flat artist crane mouse comfort thrive chimney near cradle
Network ID:        substrate
Secret seed:       0xd7eb03f23b26b2f09cc91b4b1433f4902536afbab63be6df418b6f816a4a2009
Public key (hex):  0xb54f4a2dd6a26b8c9f4749c9e936ae81d742f84ab167b40c7e8b635c5853a256
Account ID:        0xb54f4a2dd6a26b8c9f4749c9e936ae81d742f84ab167b40c7e8b635c5853a256
Public key (SS58): 5GAS5VZHniu9L87Bp97Csq6qHDQCj88bLDoLZhNSwLUuMx26
SS58 Address:      5GAS5VZHniu9L87Bp97Csq6qHDQCj88bLDoLZhNSwLUuMx26

3:
```shell
./target/release/node-template key generate --scheme Sr25519 --password-interactive
```

Secret phrase:       marriage cage infant pluck depart eternal educate garden must video deer range
Network ID:        substrate
Secret seed:       0x71e80880d942fe438a3d1a5406432da41d2a332b7fa8452c3e029c4e0576941b
Public key (hex):  0xe286e8673a00fb30a80b6ca9d854fc45da5cc32621df3b2f94f95f358fd05a2e
Account ID:        0xe286e8673a00fb30a80b6ca9d854fc45da5cc32621df3b2f94f95f358fd05a2e
Public key (SS58): 5HBikkQU3LXgHR3sgs4azmqF8MGdbkj4qmUAUqVRdGUuse6K
SS58 Address:      5HBikkQU3LXgHR3sgs4azmqF8MGdbkj4qmUAUqVRdGUuse6K

4:
```shell
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "marriage cage infant pluck depart eternal educate garden must video deer range"
```

Secret phrase:       marriage cage infant pluck depart eternal educate garden must video deer range
Network ID:        substrate
Secret seed:       0x71e80880d942fe438a3d1a5406432da41d2a332b7fa8452c3e029c4e0576941b
Public key (hex):  0x02c7373fa6c5cfa22121f7a55accfecffe651e8ef9d736aa2d22c72adec265d3
Account ID:        0x02c7373fa6c5cfa22121f7a55accfecffe651e8ef9d736aa2d22c72adec265d3
Public key (SS58): 5C8M8dcAeDhicL7fjgmFojakHPf7QjhHXdWXFX4kTWaSwWgM
SS58 Address:      5C8M8dcAeDhicL7fjgmFojakHPf7QjhHXdWXFX4kTWaSwWgM

5:
```shell
./target/release/substrate build-spec --disable-default-bootnode --chain local > customSpec.json
```

7:
```shell
./target/release/substrate \
--base-path /tmp/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode01
```

8:
```shell
./target/release/substrate key insert --base-path /tmp/node01 \
--chain customSpecRaw.json \
--scheme Sr25519 \
--suri "circle despair shy flat artist crane mouse comfort thrive chimney near cradle" \
--password-interactive \
--key-type aura

./target/release/node-template key insert --base-path /tmp/node01 \
--chain customSpecRaw.json \
--scheme Ed25519 \
--suri "circle despair shy flat artist crane mouse comfort thrive chimney near cradle" \
--password-interactive \
--key-type gran


```

9:
```shell
./target/release/substrate \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX \
--password-interactive
```


10:
```shell
./target/release/substrate key insert --base-path /tmp/node02 \
--chain customSpecRaw.json \
--scheme Sr25519 \
--suri "marriage cage infant pluck depart eternal educate garden must video deer range" \
--password-interactive \
--key-type aura

./target/release/node-template key insert --base-path /tmp/node02 \
--chain customSpecRaw.json \
--scheme Ed25519 \
--suri "marriage cage infant pluck depart eternal educate garden must video deer range" \
--password-interactive \
--key-type gran
```

11:
```shell
./target/release/substrate \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX \
--password-interactive
```



## node  有效

```shell
./target/release/substrate --port 30333 --ws-port 9944 --rpc-port 9933 --validator --ws-external --rpc-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --unsafe-ws-external --no-mdns --ws-max-connections 10000 --chain newtouch --base-path /tmp/db/node01 --name Node01 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --password p7JCjBrd
./target/release/substrate --port 30334 --ws-port 9945 --rpc-port 9934 --validator --ws-external --rpc-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --unsafe-ws-external --no-mdns --ws-max-connections 10000 --chain newtouch --base-path /tmp/db/node02 --name Node02 --node-key 0000000000000000000000000000000000000000000000000000000000000002 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp --password aOMNJMfM
./target/release/substrate --port 30335 --ws-port 9946 --rpc-port 9935 --validator --ws-external --rpc-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --unsafe-ws-external --no-mdns --ws-max-connections 10000 --chain newtouch --base-path /tmp/db/node03 --name Node03 --node-key 0000000000000000000000000000000000000000000000000000000000000003 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp --password vGe0Ov7U
./target/release/substrate --port 30336 --ws-port 9947 --rpc-port 9936 --validator --ws-external --rpc-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --unsafe-ws-external --no-mdns --ws-max-connections 10000 --chain newtouch --base-path /tmp/db/node04 --name Node04 --node-key 0000000000000000000000000000000000000000000000000000000000000004 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp --password DoLT9JwD


## 观察者节点
./target/release/substrate --port 30337 --ws-port 9948 --rpc-port 9937  --ws-external --rpc-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --unsafe-ws-external --no-mdns --ws-max-connections 10000 --chain newtouch --base-path /tmp/db/node05 --name Node05 --node-key 0000000000000000000000000000000000000000000000000000000000000005 --bootnodes /ip4/183.66.65.205/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp  --bootnodes /ip4/59.80.30.178/tcp/30333/p2p/12D3KooWSCufgHzV4fCwRijfH2k3abrpAJxTKxEvN1FDuRXA2U9x

```

### session key
- 节点1
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["babe", "excite canyon loop debate canyon tourist satoshi eyebrow discover crystal govern excuse", "0x9263aee51ec2aa1e0410793a61ca6dc799993d203c88833308e5d88a70e7673d"],"id":1 }' localhost:9933
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran", "excite canyon loop debate canyon tourist satoshi eyebrow discover crystal govern excuse", "0xe589c2b8fd9052ebafcce4aadb20f4bd0c9a269bf21eda475dfa808371c39116"],"id":1 }' localhost:9933

## 节点2
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["babe", "fiction only nasty stay angry bean hour broccoli doctor fade pelican stuff", "0xee0087f07259f60f45400d2f59751da9a548cca8e255290c3295c5017f4b5a27"],"id":1 }' localhost:9934
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran","fiction only nasty stay angry bean hour broccoli doctor fade pelican stuff", "0x717d832e4065bd28389cfcf9a7d5a18474491acd69bce3d0a0cf174cc11ea33f"],"id":1 }' localhost:9934

## 节点3
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["babe", "chunk nest ask shop upset over van enjoy airport way critic garage", "0x006c94019cc88c8404cf14c93c0135436ef9cce85c0afc3f4c007ba811213210"],"id":1 }' localhost:9935
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran","chunk nest ask shop upset over van enjoy airport way critic garage", "0xe723037e2e9d3daebeedc8f78d19adf62ec781645eff27a74a29e19c3a2c583a"],"id":1 }' localhost:9935

## 节点4
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["babe", "arm promote liar spell aim auto decrease excess acid blouse glue mobile", "0x1c7efaf38edc4c836352af11edd67e769998ae0680c63a08f1916581dabd4d54"],"id":1 }' localhost:9936
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran", "arm promote liar spell aim auto decrease excess acid blouse glue mobile", "0xfa7647a9184d06f29a630ada50b11ee14cd7b784ee74ea7297fc5f1e64a44dbd"],"id":1 }' localhost:9936
```