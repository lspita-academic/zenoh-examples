# Notes

## Peers seem to not see each other

Build in debug mode and launch with `RUST_LOG=zenoh=debug` to see debug logs: often the problem is not that they see each other (you can see in the logs they find the associated zid of each other) but that the firewall is not open. Specify the multicast and listen ports and open them on the firewall.

```sh
# -I: insert at the top of the chain (so that it's the first rule match)
# -D: remove from chain
sudo iptables -I INPUT -p udp --dport 7446 -j ACCEPT
sudo iptables -I INPUT -p udp --dport 7447 -j ACCEPT
sudo iptables -I INPUT -p tcp --dport 7446 -j ACCEPT
sudo iptables -I INPUT -p tcp --dport 7447 -j ACCEPT
```

## Normal zenoh and zenoh-pico iteropability

https://github.com/eclipse-zenoh/zenoh-pico?tab=readme-ov-file#error-when-opening-a-session-on-a-microcontroller

set batch sizes to match (better to reduce on normal zenoh than to increase in pico)
