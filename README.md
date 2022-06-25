# Directory Contents
- [Infrastructure as Code](./iac)
- [Telemetry Pub Sub Code](./tps)

# General Notes
## Adding a security group on Chameleon via CLI
```
openstack security group rule create --protocol sctp --ingress --dst-port 38412 5g
```
