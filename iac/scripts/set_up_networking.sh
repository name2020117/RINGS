sudo sysctl -w net.ipv4.ip_forward=1;
sudo iptables -t nat -A POSTROUTING -o ens3 -j MASQUERADE;
sudo systemctl stop ufw;
