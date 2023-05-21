#!/bin/bash
qemu-system-x86_64 -boot d \
      -enable-kvm \
      -smp 2 \
	-nodefaults \
	-nographic \
      -name fedora \
      -cpu host \
      -m 3G \
      -drive file=./Fedora-Server-KVM-38-1.6.x86_64.qcow2,if=virtio,aio=native,cache.direct=on,cache=writeback \
      -object rng-random,id=rng0,filename=/dev/urandom \
      -device virtio-rng-pci,rng=rng0 \
      -serial pty \
      -monitor stdio \
      -device virtio-balloon \
      -netdev nic,id=mynet0,type=user,hostfwd=tcp::9090-:9090\
      -fsdev local,security_model=none,id=fsdev0,path=./../ -device virtio-9p-pci,id=fs0,fsdev=fsdev0,mount_tag=hostshare \
      -device virtio-net-pci,netdev=mynet0,mac=52:55:00:d1:56:01
