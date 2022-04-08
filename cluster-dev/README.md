# High Availability and High Performance Computing With Fluence AClusters

## Overview

### Definition

* An  ACluster is a group of Fluence peers and target services (?), i.e. [(peer, (s1, s2, ..., sk), ...], that can be programmed to provide both availability, i.e., failover, and high-performance compute, i.e. parallelization, capabilities.
* Aqua is used to program the ACluster lifecyle:
  * setup an configuration, 
  * runtime management including
    * leader selection,
    * member replacement or expansion
    * health and performance monitoring
  * teardown

### Use Cases

* Failover
* Parallelization
* Poor man's subnetwork, e.g., dedicated TSS, storage, TEE or GPU provider peers


### Note to Self

We should look into providing a pattern to allow developers to convert concurrent programs, e.g., NLP, into single threaded wasm modules where Aqua + ACluster provides the availability and dynamic concurrency.


## Approach

Want to:

* use Aqua-Registry to manage ACluster
* use leader selection ranging from manual to random to Raft selection
* maintain and sync ACluster state, e.g., sqlite file, or use Ceramic, Textile, etc.
* scale and autoscale ACluster members, i.e., add new (peer + services) to ACluster to replace failed, terminated or withdrawn members or to improve performance, i.e., parallelization.
  * what permissioning is needed from peers? should be enough if a peer is willing to host/already hosts all the services required by ACluster
  * Bringing economics and licenses into it shouldn't be too bad
* monitor and heal cluster
  * availability
  * performance



### Structs

```python
data ServiceInstance:
    blueprint_id: string
    cid: ?string
    service_id: ?string
    -- license_id: string

data ServiceSet:
    id: string
    services: *ServiceInstance

data Server:
    peer_id: string
    relay_id: string
    services: *ServiceSet

 data Cluster:
    cluster_id: string
    cluster_name: string
    server: *Server

```