#!/bin/bash

# Require jq installed:
#  brew install jq
#  apt install jq

function req {
  rand_number=$((RANDOM))
  data="${2}"
  if [ -z "${data}" ]; then
    data='{}'
  fi
  curl -sgX POST -H "Content-Type: application/json" "http://127.0.0.1:8000/${1}" \
       -d "{\"input\":${data}, \"nonce\": {\"id\": ${rand_number}}}" \
       | tee /tmp/req_result.json | jq '.payload|fromjson'
  echo
}

function req_set {
  id=$1
  path=$2
  file_b64=$(cat "${path}"| base64 -w 0)
  req set "{\"path\": \"${id}\", \"data\": \"${file_b64}\"}"
}

function set_dataset {
  req_set "/ipfs/QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG" ./scripts/dataset.csv
}

function set_query {
  req_set "/ipfs/QmY6yj1GsermExDXoosVE3aSPxdMNYr6aKuw3nA8LoWPRS" ./scripts/query.csv
}

function get_result {
  path="/order/0"
  req get "{\"path\": \"${path}\"}"
  cat /tmp/req_result.json | jq '.payload|fromjson|.value' -r | base64 -d
}

case $1 in
set-dataset)
  set_dataset
;;
set-query)
  set_query
;;
set-all)
  set_dataset
  set_query
;;
get-result)
  get_result
;;
*)
  req "$@"
;;
esac