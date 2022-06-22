# Repository Contents
- [Infrastructure as Code](./iac)
- [Telemetry Pub Sub implementation](./tps)

# Instructions
## Free5gc on VM
(1) Change parameters in [variable file]() to match Chameleon Cloud VMs that were created.
For the most part, this is updating internal IP addresses (this will need to be automated eventually)

(2) Run playbook locally
  ```sh
  ansible-playbook core_playbook.yaml
  ```
(2) SSH into the VM

(3) Build free5gc with script
``` sh
sh make_free5gc.sh
```

(4) Set up free5gc networking \
NOTE: might need to change network interface if not on chameleon VM
``` sh
sh set_up_networking.sh
```

(5) Test free5gc \
NOTE: script needs to be improved to either print failed tests at end, or just stop.
In meantime, one can search through terminal trace for any failures.
``` sh
cd /home/cc/free5gc;
sudo sh test_all_free5gc.sh
```

(6) Start webconsole, and [make any changes like adding a subscriber](https://www.free5gc.org/installations/stage-3-sim-install/)
``` sh
cd /home/cc/free5gc/webconsole
go run server.go
```

(7) Set up forwarding
```sh
sudo iptables -I FORWARD 1 -j ACCEPT
```

(8) Start free5gc
```sh
cd /home/cc/free5gc
./run.sh
```

## UERANSIM on VM
(1) Change parameters in [variable file]() to match Chameleon Cloud VMs that were created

(2) Run playbook locally
  ```sh
  ansible-playbook ran_playbook.yaml
  ```

(2) SSH into the VM
(3) Start nr-gnb
```sh
cd ~/UERANSIM
build/nr-gnb -c config/free5gc-gnb.yaml
```
(4) Execute nr-gnb with admin right
```sh
cd ~/UERANSIM
sudo build/nr-ue -c config/free5gc-ue.yaml
```
(5) At this point, one should be able to test the connection, with say ```ping -I uesimtun0 google.com```, and it should be working.
Next, one can run an experiment, like in the [telemetry pub sub example](./tps)
