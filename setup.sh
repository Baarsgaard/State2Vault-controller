#!/bin/bash

if ! kind --version &>/dev/null; then
  path="$HOME/.local/bin"
  mkdir "$path" &>/dev/null || :

  [ $(uname -m) = x86_64 ] && curl -fsSLo "$path/kind" https://kind.sigs.k8s.io/dl/v0.23.0/kind-linux-amd64
  [ $(uname -m) = aarch64 ] && curl -fsSLo "$path/kind" https://kind.sigs.k8s.io/dl/v0.23.0/kind-linux-arm64
  chmod 0550 "$path/kind"
fi

if [[ "$1" != *"-d"* ]]; then
  kind create cluster --config kind-config.yml
  kubectl config set-context kind-oec
else
  kind delete clusters oec
fi
