#!/bin/bash

KOURIER_CONTROL_NAMESPACE=knative-serving
out_dir="$(mktemp -d /tmp/certs-XXX)"
subdomain="example.com"

openssl req -x509 -sha256 -nodes -days 365 -newkey rsa:2048 \
  -subj "/O=Example Inc./CN=Example" \
  -keyout "${out_dir}"/root.key \
  -out "${out_dir}"/root.crt

openssl req -nodes -newkey rsa:2048 \
    -subj "/O=Example Inc./CN=Example" \
    -reqexts san \
    -config <(printf "[req]\ndistinguished_name=req\n[san]\nsubjectAltName=DNS:*.%s" "$subdomain") \
    -keyout "${out_dir}"/wildcard.key \
    -out "${out_dir}"/wildcard.csr

openssl x509 -req -days 365 -set_serial 0 \
    -extfile <(printf "subjectAltName=DNS:*.%s" "$subdomain") \
    -CA "${out_dir}"/root.crt \
    -CAkey "${out_dir}"/root.key \
    -in "${out_dir}"/wildcard.csr \
    -out "${out_dir}"/wildcard.crt
