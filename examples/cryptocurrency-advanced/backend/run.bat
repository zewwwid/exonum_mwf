mkdir example
exonum_mwf generate-template example/common.toml --validators-count 4

exonum_mwf generate-config example/common.toml  example/1 --peer-address 127.0.0.1:6331 -n
exonum_mwf generate-config example/common.toml  example/2 --peer-address 127.0.0.1:6332 -n
exonum_mwf generate-config example/common.toml  example/3 --peer-address 127.0.0.1:6333 -n
exonum_mwf generate-config example/common.toml  example/4 --peer-address 127.0.0.1:6334 -n

exonum_mwf finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/1/sec.toml example/node1.toml --public-configs example/1/pub.toml example/2/pub.toml example/3/pub.toml example/4/pub.toml
exonum_mwf finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/2/sec.toml example/node2.toml --public-configs example/1/pub.toml example/2/pub.toml example/3/pub.toml example/4/pub.toml
exonum_mwf finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/3/sec.toml example/node3.toml --public-configs example/1/pub.toml example/2/pub.toml example/3/pub.toml example/4/pub.toml
exonum_mwf finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/4/sec.toml example/node4.toml --public-configs example/1/pub.toml example/2/pub.toml example/3/pub.toml example/4/pub.toml


start exonum_mwf run --node-config example/node1.toml --db-path example/1/db --public-api-address 0.0.0.0:8200 --consensus-key-pass pass --service-key-pass pass
start exonum_mwf run --node-config example/node2.toml --db-path example/2/db --public-api-address 0.0.0.0:8201 --consensus-key-pass pass --service-key-pass pass
start exonum_mwf run --node-config example/node3.toml --db-path example/3/db --public-api-address 0.0.0.0:8202 --consensus-key-pass pass --service-key-pass pass
start exonum_mwf run --node-config example/node4.toml --db-path example/4/db --public-api-address 0.0.0.0:8203 --consensus-key-pass pass --service-key-pass pass
