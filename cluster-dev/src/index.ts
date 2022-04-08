import { Fluence, FluencePeer, KeyPair as FluenceKeyPair, KeyPair } from "@fluencelabs/fluence";
import { krasnodar, Node } from "@fluencelabs/fluence-network-environment";
import { Buffer } from 'buffer';
import { Multiaddr, protocols } from 'multiaddr';
import { put, get_from, set_timeout, get_external_api_multiaddr } from "./_generated/exports";
import { just_hello } from "./_generated/hello";
// const { create } = require('ipfs-http-client');

// import { remove_service } from "./_generated/service_handler";

import { createRoute, registerForRoute, resolveRoute, getRouteId } from "./_generated/exports";

/*

aqua key create
{
    "peerId": "12D3KooWFxbCHz9uaKx6vVbS9VcZXrb2mdzApwBJ4UjABqUFdwVB",
    "secretKey": "cy9mGOQ/pa8SqmEyOYTcfmHOWBD8yNh5dTGyWA01uwo=",
    "publicKey": "CAESIFtBf1RCF6cmDmHRS7UOsizrZYeC7BiiR9RdlFxMha5a"
}


aqua remote deploy_service --addr /dns4/kras-09.fluence.dev/tcp/19001/wss/p2p/12D3KooWD7CvsYcpF9HE9CCV9aY3SJ317tkXVykjtZnht2EbzDPm  --data-path configs/simple_service_deploy.json --service simple-service --sk cy9mGOQ/pa8SqmEyOYTcfmHOWBD8yNh5dTGyWA01uwo= --show-config
{
    "peerId": "12D3KooWFxbCHz9uaKx6vVbS9VcZXrb2mdzApwBJ4UjABqUFdwVB",
    "secretKey": "cy9mGOQ/pa8SqmEyOYTcfmHOWBD8yNh5dTGyWA01uwo=",
    "publicKey": "CAESIFtBf1RCF6cmDmHRS7UOsizrZYeC7BiiR9RdlFxMha5a",
    "relay": "/dns4/kras-09.fluence.dev/tcp/19001/wss/p2p/12D3KooWD7CvsYcpF9HE9CCV9aY3SJ317tkXVykjtZnht2EbzDPm",
    "timeout": 60000,
    "log-level": "INFO"
}
Going to upload a module...
2022.03.31 17:48:31 [INFO] created ipfs client to /ip4/164.90.171.156/tcp/5001
2022.03.31 17:48:31 [INFO] connected to ipfs
2022.03.31 17:48:34 [INFO] file uploaded
Now time to make a blueprint...
Blueprint id:
94a628b5e18df76b7126002585bf3e85eb61ea6ff7bb716564a53a98c92bcf00
And your service id is:
"f4f728ee-4a3c-4873-94b9-92ec5c3cdbd5"

*/


// temp fix replace with your key, e.g., account pk
const SeedArray = new Uint8Array([10, 10, 20, 20, 100, 100, 10, 10, 20, 20, 100, 100, 10, 10, 10, 10, 10, 10, 20, 20, 100, 100, 10, 10, 20, 20, 100, 100, 10, 10, 10, 10]);


async function main() {
    const kp = await KeyPair.fromEd25519SK(SeedArray);

    let peer = new FluencePeer;
    let peer_started = await peer.start({ connectTo: krasnodar[3], KeyPair: kp });
    console.log("peer started: ", peer.getStatus());



    console.log("ctrl-c to exit");
    await peer.stop();
}


main();
