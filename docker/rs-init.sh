#!/bin/bash

mongo --host mongodb:27017 <<EOF
rs.initiate({ _id: "rs0", version: 1, members: [ { _id: 0, host: "mongodb:27017" } ] });
EOF