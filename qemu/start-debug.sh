#!/bin/bash
qemu-system-x86_64 -boot d \
      -smp 4 \
      -accel kvm\
	-nodefaults \
	-nographic \
      -name fedora \
      -cpu host \
      -m 4G \
      -s -S\
      -drive file=./Fedora-Server-KVM-38-1.6.x86_64.qcow2,if=virtio,aio=native,cache.direct=on,cache=writeback \
      -object rng-random,id=rng0,filename=/dev/urandom \
      -device virtio-rng-pci,rng=rng0 \
      -serial stdio \
      -monitor telnet::45454,server,nowait \
      -device virtio-balloon \
      -netdev nic,id=mynet0,type=user,hostfwd=tcp::9090-:9090,hostfwd=tcp::10022-:22\
      -fsdev local,security_model=none,id=fsdev0,path=./../ -device virtio-9p-pci,id=fs0,fsdev=fsdev0,mount_tag=hostshare \
      -device virtio-net-pci,netdev=mynet0,mac=52:55:00:d1:56:01
