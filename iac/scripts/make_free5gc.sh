# Needed because kernell updates mess up gtp5g
KERNEL=$(uname -r)
sudo apt install -y yarn linux-headers-$KERNEL
make --directory free5gc;
make --directory gtp5g;
sudo make --directory gtp5g install;
make --directory free5gc webconsole;
