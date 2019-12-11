#!/bin/bash

function req {
  rand_number=$((RANDOM))
  data="${2}"
  if [ -z "${data}" ]; then
    data='{}'
  fi
  curl -X POST -H "Content-Type: application/json" "http://127.0.0.1:8000/${1}" \
       -d "{\"input\":${data}, \"nonce\": {\"id\": ${rand_number}}}"
  echo
}

case $1 in
*)
  req "$@"
;;
esac