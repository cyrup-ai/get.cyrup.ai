#!/bin/bash
set -e

# Install k3s
curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='--disable traefik' sh -s -

# Setup kubeconfig
mkdir -p /home/omniforge/.kube
k3s kubectl config view --raw > /home/omniforge/.kube/config
chown -R omniforge:omniforge /home/omniforge/.kube

# Install helm
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
