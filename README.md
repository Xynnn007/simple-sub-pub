# simple-sub-pub

This is a simple demo of subscriber-publisher model, using 
`tokio`.

## Prepare

We need redis. Suppose the following operations all happen in a container.

```bash
apt install redis-server
redis-server
```

Then the redis server will be launched.

```bash
                _._                                                  
           _.-``__ ''-._                                             
      _.-``    `.  `_.  ''-._           Redis 6.0.16 (00000000/0) 64 bit
  .-`` .-```.  ```\/    _.,_ ''-._                                   
 (    '      ,       .-`  | `,    )     Running in standalone mode
 |`-._`-...-` __...-.``-._|'` _.-'|     Port: 6379
 |    `-._   `._    /     _.-'    |     PID: 3711756
  `-._    `-._  `-./  _.-'    _.-'                                   
 |`-._`-._    `-.__.-'    _.-'_.-'|                                  
 |    `-._`-._        _.-'_.-'    |           http://redis.io        
  `-._    `-._`-.__.-'_.-'    _.-'                                   
 |`-._`-._    `-.__.-'    _.-'_.-'|                                  
 |    `-._`-._        _.-'_.-'    |                                  
  `-._    `-._`-.__.-'_.-'    _.-'                                   
      `-._    `-.__.-'    _.-'                                       
          `-._        _.-'                                           
              `-.__.-'                                               
```

Then, we need to set the redis port for the `pub` and `sub`.
As `redis-server` claims, the port is `6379`. In `src/bin/pub.rs` and `src/bin/sub.rs`, set `.addr("redis://localhost:6379")`.

Then launch the `sub`.
```bash
cargo run --bin sub
```

The terminal will block, and wait for the publisher's messages to come.

Open another terminal, launch `pub`.

```bash
cargo run --bin pub
```

The `pub` will immidiately finish. At the same time,
the `sub`'s terminal will print as following
```bash
get 321
```