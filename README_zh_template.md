# 本地启动substrate集群
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