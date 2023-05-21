#!/bin/bash
qemu-system-x86_64 \
-enable-kvm \
-M q35 \
-m 4096 -smp 4 -cpu host \
-drive file=./Fedora-Server-KVM-38-1.6.x86_64.qcow2,if=virtio \
-device virtio-tablet \
-device virtio-keyboard \
-machine vmport=off \
-device virtio-vga-gl,edid=true -display sdl,gl=on \
-audiodev pa,id=snd0 -device AC97,audiodev=snd0 \
-net nic,model=virtio-net-pci -net user,hostfwd=tcp::4444-:5555
